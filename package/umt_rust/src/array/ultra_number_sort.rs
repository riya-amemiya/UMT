/// Ultra-fast sorting specifically optimized for number arrays
///
/// # Arguments
/// * `array` - Array of numbers to sort
/// * `ascending` - Sort in ascending order if true, descending if false
///
/// # Returns
/// Sorted array
#[inline]
pub fn umt_ultra_number_sort(array: &mut Vec<f64>, ascending: bool) -> &mut Vec<f64> {
    let length = array.len();

    if length <= 1 {
        return array;
    }

    if length == 2 {
        if (array[0] > array[1]) == ascending {
            array.swap(0, 1);
        }
        return array;
    }

    if length == 3 {
        inline_sort3(array, ascending);
        return array;
    }

    // Cheap linear probe: NaN presence + already-ordered / reversed input.
    // Pays for itself on sorted/reversed data and keeps the hot random path
    // from doing integer analysis when radix/comparison is the real work.
    let (has_nan, sorted_asc, sorted_desc) = probe_order(array);
    if !has_nan && try_already_ordered(array, ascending, sorted_asc, sorted_desc) {
        return array;
    }

    // Small-medium arrays: skip integer/range analysis
    if length <= 128 {
        if has_nan {
            return handle_nan_sort(array, ascending);
        }
        comparison_sort(array, ascending);
        return array;
    }

    // Integer range analysis for counting-sort eligibility
    let mut all_integers = true;
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    if !has_nan {
        for &value in array.iter() {
            if all_integers {
                if value.fract() != 0.0 {
                    all_integers = false;
                } else {
                    if value < min {
                        min = value;
                    }
                    if value > max {
                        max = value;
                    }
                }
            } else {
                break;
            }
        }
    } else {
        all_integers = false;
    }

    // For small integer ranges, use counting sort
    if !has_nan && all_integers && max - min < (length * 2) as f64 && max - min < 1_000_000.0 {
        return counting_sort(array, min, max, ascending);
    }

    // Below the crossover, comparison sort beats radix sort because the radix
    // path pays a fixed cost (buffer allocation + LSD passes). Measured
    // crossover sits around 4096 for both integer and floating-point input.
    const RADIX_THRESHOLD: usize = 4096;
    if length < RADIX_THRESHOLD {
        if has_nan {
            return handle_nan_sort(array, ascending);
        }
        comparison_sort(array, ascending);
        return array;
    }

    float64_radix_sort(array, ascending, has_nan);
    array
}

/// Probe for NaN and whether the array is already sorted ascending/descending.
#[inline]
fn probe_order(array: &[f64]) -> (bool, bool, bool) {
    let mut sorted_asc = true;
    let mut sorted_desc = true;
    let mut prev = array[0];
    if prev.is_nan() {
        return (true, false, false);
    }
    for &value in &array[1..] {
        if value.is_nan() {
            return (true, false, false);
        }
        if value < prev {
            sorted_asc = false;
        }
        if value > prev {
            sorted_desc = false;
        }
        prev = value;
    }
    (false, sorted_asc, sorted_desc)
}

#[inline]
fn try_already_ordered(
    array: &mut [f64],
    ascending: bool,
    sorted_asc: bool,
    sorted_desc: bool,
) -> bool {
    if ascending {
        if sorted_asc {
            return true;
        }
        if sorted_desc {
            array.reverse();
            return true;
        }
    } else if sorted_desc {
        return true;
    } else if sorted_asc {
        array.reverse();
        return true;
    }
    false
}

/// Inline sort for 3 elements
#[inline]
fn inline_sort3(array: &mut [f64], ascending: bool) {
    let mut a = array[0];
    let mut b = array[1];
    let mut c = array[2];

    if ascending {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        if b > c {
            std::mem::swap(&mut b, &mut c);
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
        }
    } else {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        if b < c {
            std::mem::swap(&mut b, &mut c);
            if a < b {
                std::mem::swap(&mut a, &mut b);
            }
        }
    }

    array[0] = a;
    array[1] = b;
    array[2] = c;
}

/// Handle arrays with NaN values (NaNs go to the end)
fn handle_nan_sort(array: &mut Vec<f64>, ascending: bool) -> &mut Vec<f64> {
    let mut write = 0;
    let len = array.len();
    for read in 0..len {
        if !array[read].is_nan() {
            if write != read {
                array[write] = array[read];
            }
            write += 1;
        }
    }
    let nan_count = len - write;
    array.truncate(write);
    comparison_sort(array, ascending);
    array.resize(write + nan_count, f64::NAN);
    array
}

/// Counting sort for small integer ranges
fn counting_sort(array: &mut Vec<f64>, min: f64, max: f64, ascending: bool) -> &mut Vec<f64> {
    let range = (max - min + 1.0) as usize;
    let mut count = vec![0u32; range];

    for &element in array.iter() {
        count[(element - min) as usize] += 1;
    }

    let mut index = 0;
    if ascending {
        for (i, &cnt) in count.iter().enumerate() {
            let value = i as f64 + min;
            let end = index + cnt as usize;
            array[index..end].fill(value);
            index = end;
        }
    } else {
        for i in (0..range).rev() {
            let cnt = count[i];
            let value = i as f64 + min;
            let end = index + cnt as usize;
            array[index..end].fill(value);
            index = end;
        }
    }

    array
}

/// Transform IEEE 754 f64 bits so that unsigned integer order matches
/// numerical order (NaNs should be filtered first).
#[inline]
fn to_sortable_bits(v: f64) -> u64 {
    let bits = v.to_bits();
    if (bits as i64) < 0 {
        !bits
    } else {
        bits ^ 0x8000_0000_0000_0000
    }
}

/// Reverse the sortable-bit transform back to a floating-point value.
#[inline]
fn from_sortable_bits(bits: u64) -> f64 {
    let bits = if (bits as i64) < 0 {
        bits ^ 0x8000_0000_0000_0000
    } else {
        !bits
    };
    f64::from_bits(bits)
}

/// IEEE 754 Float64 LSD radix sort using 11-bit digits (6 passes).
/// Works on integers and floats by sorting the transformed bit patterns.
fn float64_radix_sort(array: &mut [f64], ascending: bool, has_nan: bool) {
    const RADIX_BITS: u32 = 11;
    const RADIX: usize = 1 << RADIX_BITS;
    const PASSES: usize = 6;
    const MASK: u64 = (RADIX as u64) - 1;

    let length = array.len();
    let mut source = vec![0u64; length];
    let mut histograms = vec![0u32; PASSES * RADIX];

    let valid_length = if has_nan {
        let mut write_index = 0;
        for &v in array.iter() {
            if v.is_nan() {
                continue;
            }
            let bits = to_sortable_bits(v);
            source[write_index] = bits;
            histograms[(bits & MASK) as usize] += 1;
            histograms[RADIX + ((bits >> 11) & MASK) as usize] += 1;
            histograms[2 * RADIX + ((bits >> 22) & MASK) as usize] += 1;
            histograms[3 * RADIX + ((bits >> 33) & MASK) as usize] += 1;
            histograms[4 * RADIX + ((bits >> 44) & MASK) as usize] += 1;
            histograms[5 * RADIX + ((bits >> 55) & 0x1ff) as usize] += 1;
            write_index += 1;
        }
        write_index
    } else {
        for (index, &v) in array.iter().enumerate() {
            let bits = to_sortable_bits(v);
            source[index] = bits;
            histograms[(bits & MASK) as usize] += 1;
            histograms[RADIX + ((bits >> 11) & MASK) as usize] += 1;
            histograms[2 * RADIX + ((bits >> 22) & MASK) as usize] += 1;
            histograms[3 * RADIX + ((bits >> 33) & MASK) as usize] += 1;
            histograms[4 * RADIX + ((bits >> 44) & MASK) as usize] += 1;
            histograms[5 * RADIX + ((bits >> 55) & 0x1ff) as usize] += 1;
        }
        length
    };

    if valid_length == 0 {
        return;
    }

    source.truncate(valid_length);
    let mut destination = vec![0u64; valid_length];

    let mut current_source = &mut source[..];
    let mut current_destination = &mut destination[..];

    for pass in 0..PASSES {
        let base = pass * RADIX;
        let shift = pass as u32 * RADIX_BITS;
        let digit_mask = if pass == PASSES - 1 { 0x1ff } else { MASK };
        let bucket_count = (digit_mask as usize) + 1;

        let mut skip_pass = false;
        for index in 0..bucket_count {
            if histograms[base + index] == valid_length as u32 {
                skip_pass = true;
                break;
            }
        }
        if skip_pass {
            continue;
        }

        if ascending {
            for index in 1..bucket_count {
                histograms[base + index] += histograms[base + index - 1];
            }
        } else {
            for index in (0..bucket_count - 1).rev() {
                histograms[base + index] += histograms[base + index + 1];
            }
        }

        for index in (0..valid_length).rev() {
            let bits = current_source[index];
            let digit = ((bits >> shift) & digit_mask) as usize;
            histograms[base + digit] -= 1;
            let pos = histograms[base + digit] as usize;
            current_destination[pos] = bits;
        }

        std::mem::swap(&mut current_source, &mut current_destination);
    }

    for (slot, &bits) in array.iter_mut().zip(current_source.iter()) {
        *slot = from_sortable_bits(bits);
    }
    array[valid_length..].fill(f64::NAN);
}

/// Comparison-based sort used below the radix threshold.
/// Uses the stdlib unstable sort, which is typically faster than a
/// hand-rolled quicksort on modern Rust for general floating-point input.
#[inline]
fn comparison_sort(array: &mut [f64], ascending: bool) {
    if ascending {
        array.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    } else {
        array.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    }
}
