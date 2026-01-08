/// Ultra-fast sorting specifically optimized for number arrays
///
/// # Arguments
/// * `array` - Array of numbers to sort
/// * `ascending` - Sort in ascending order if true, descending if false
///
/// # Returns
/// Sorted array
pub fn umt_ultra_number_sort(array: &mut Vec<f64>, ascending: bool) -> &mut Vec<f64> {
    let length = array.len();

    if length <= 1 {
        return array;
    }

    // For tiny arrays, use optimized inline sort
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

    // Check if all numbers are integers and find range
    let mut all_integers = true;
    let mut min = array[0];
    let mut max = array[0];
    let mut has_nan = false;

    for &value in array.iter() {
        if value.is_nan() {
            has_nan = true;
            break;
        }
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
        if all_integers && value.fract() != 0.0 {
            all_integers = false;
        }
    }

    // Handle NaN values
    if has_nan {
        return handle_nan_sort(array, ascending);
    }

    // For small integer ranges, use counting sort
    if all_integers && max - min < (length * 2) as f64 && max - min < 1_000_000.0 {
        return counting_sort(array, min, max, ascending);
    }

    // For larger arrays, use radix sort if applicable
    if all_integers && length > 100 {
        return radix_sort(array, ascending);
    }

    // Fall back to optimized quicksort for floating point
    numeric_quick_sort(array, 0, (length - 1) as isize, ascending);
    array
}

/// Inline sort for 3 elements
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

/// Handle arrays with NaN values
fn handle_nan_sort(array: &mut Vec<f64>, ascending: bool) -> &mut Vec<f64> {
    let mut valid: Vec<f64> = Vec::new();
    let mut nan_count = 0;

    for &element in array.iter() {
        if !element.is_nan() {
            valid.push(element);
        } else {
            nan_count += 1;
        }
    }

    let valid_len = valid.len() as isize;
    numeric_quick_sort(&mut valid, 0, valid_len - 1, ascending);

    // NaN values go to the end
    for _ in 0..nan_count {
        valid.push(f64::NAN);
    }

    // Copy back
    *array = valid;
    array
}

/// Counting sort for small integer ranges
fn counting_sort(array: &mut Vec<f64>, min: f64, max: f64, ascending: bool) -> &mut Vec<f64> {
    let range = (max - min + 1.0) as usize;
    let mut count = vec![0u32; range];

    // Count occurrences
    for &element in array.iter() {
        count[(element - min) as usize] += 1;
    }

    // Reconstruct array
    let mut index = 0;
    if ascending {
        for (i, &cnt) in count.iter().enumerate().take(range) {
            let value = i as f64 + min;
            for _ in 0..cnt {
                array[index] = value;
                index += 1;
            }
        }
    } else {
        for i in (0..range).rev() {
            let cnt = count[i];
            let value = i as f64 + min;
            for _ in 0..cnt {
                array[index] = value;
                index += 1;
            }
        }
    }

    array
}

/// Radix sort for integers
fn radix_sort(array: &mut Vec<f64>, ascending: bool) -> &mut Vec<f64> {
    // Separate positive and negative numbers
    let mut positive: Vec<f64> = Vec::new();
    let mut negative: Vec<f64> = Vec::new();
    let mut zero_count = 0;

    for &value in array.iter() {
        if value > 0.0 {
            positive.push(value);
        } else if value < 0.0 {
            negative.push(-value);
        } else {
            zero_count += 1;
        }
    }

    // Sort positive numbers
    if !positive.is_empty() {
        radix_sort_positive(&mut positive);
    }

    // Sort negative numbers
    if !negative.is_empty() {
        radix_sort_positive(&mut negative);
    }

    // Merge results
    array.clear();
    if ascending {
        // Negative numbers first (in reverse order)
        for i in (0..negative.len()).rev() {
            array.push(-negative[i]);
        }
        // Zeros
        for _ in 0..zero_count {
            array.push(0.0);
        }
        // Positive numbers
        array.extend(positive);
    } else {
        // Positive numbers first (in reverse order)
        for i in (0..positive.len()).rev() {
            array.push(positive[i]);
        }
        // Zeros
        for _ in 0..zero_count {
            array.push(0.0);
        }
        // Negative numbers
        for value in negative {
            array.push(-value);
        }
    }

    array
}

/// Radix sort for positive integers
fn radix_sort_positive(array: &mut [f64]) {
    let length = array.len();
    if length <= 1 {
        return;
    }

    // Find maximum to determine number of digits
    let max = array.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)) as u64;

    // Use vectors for output and count
    let mut output = vec![0.0; length];
    let mut count = vec![0usize; 256];

    // Process 8 bits at a time
    let mut shift = 0;
    while max >> shift > 0 {
        // Reset count array
        count.fill(0);

        // Count occurrences
        for &value in array.iter() {
            let digit = ((value as u64 >> shift) & 0xff) as usize;
            count[digit] += 1;
        }

        // Change count[i] to actual position
        for i in 1..256 {
            count[i] += count[i - 1];
        }

        // Build output array
        for i in (0..length).rev() {
            let digit = ((array[i] as u64 >> shift) & 0xff) as usize;
            count[digit] -= 1;
            output[count[digit]] = array[i];
        }

        // Copy back
        array.copy_from_slice(&output);

        shift += 8;
    }
}

/// Optimized quicksort for numbers
fn numeric_quick_sort(array: &mut [f64], low: isize, high: isize, ascending: bool) {
    let mut stack = Vec::new();
    stack.push((low, high));

    while let Some((l, h)) = stack.pop() {
        if h <= l {
            continue;
        }

        // For small subarrays, use insertion sort
        if h - l < 16 {
            numeric_insertion_sort(array, l as usize, h as usize, ascending);
            continue;
        }

        // Partition
        let pivot = numeric_partition(array, l as usize, h as usize, ascending);

        // Push larger partition first to limit stack depth
        if pivot as isize - l > h - pivot as isize {
            stack.push((l, pivot as isize - 1));
            stack.push((pivot as isize + 1, h));
        } else {
            stack.push((pivot as isize + 1, h));
            stack.push((l, pivot as isize - 1));
        }
    }
}

/// Numeric insertion sort
fn numeric_insertion_sort(array: &mut [f64], low: usize, high: usize, ascending: bool) {
    if ascending {
        for i in (low + 1)..=high {
            let key = array[i];
            let mut j = i as isize - 1;
            while j >= low as isize && array[j as usize] > key {
                array[(j + 1) as usize] = array[j as usize];
                j -= 1;
            }
            array[(j + 1) as usize] = key;
        }
    } else {
        for i in (low + 1)..=high {
            let key = array[i];
            let mut j = i as isize - 1;
            while j >= low as isize && array[j as usize] < key {
                array[(j + 1) as usize] = array[j as usize];
                j -= 1;
            }
            array[(j + 1) as usize] = key;
        }
    }
}

/// Numeric partition with median-of-three pivot
fn numeric_partition(array: &mut [f64], low: usize, high: usize, ascending: bool) -> usize {
    // Median-of-three pivot selection
    let mid = low + ((high - low) >> 1);

    if ascending {
        if array[mid] < array[low] {
            array.swap(low, mid);
        }
        if array[high] < array[low] {
            array.swap(low, high);
        }
        if array[high] < array[mid] {
            array.swap(mid, high);
        }
    } else {
        if array[mid] > array[low] {
            array.swap(low, mid);
        }
        if array[high] > array[low] {
            array.swap(low, high);
        }
        if array[high] > array[mid] {
            array.swap(mid, high);
        }
    }

    // Move pivot to end-1
    array.swap(mid, high - 1);
    let pivot = array[high - 1];

    let mut i = low;
    let mut j = high - 1;

    if ascending {
        loop {
            i += 1;
            while array[i] < pivot {
                i += 1;
            }
            j -= 1;
            while array[j] > pivot {
                j -= 1;
            }
            if i >= j {
                break;
            }
            array.swap(i, j);
        }
    } else {
        loop {
            i += 1;
            while array[i] > pivot {
                i += 1;
            }
            j -= 1;
            while array[j] < pivot {
                j -= 1;
            }
            if i >= j {
                break;
            }
            array.swap(i, j);
        }
    }

    array.swap(i, high - 1);
    i
}
