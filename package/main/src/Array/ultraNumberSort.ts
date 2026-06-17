/**
 * Ultra-fast sorting specifically optimized for number arrays
 * @param array Array of numbers to sort
 * @param ascending Sort in ascending order if true, descending if false
 * @returns Sorted array
 */
export const ultraNumberSort = (
  array: number[],
  ascending = true,
): number[] => {
  const length = array.length;

  if (length <= 1) {
    return [...array];
  }

  const result = [...array];

  if (length === 2) {
    if (result[0] > result[1] === ascending) {
      const temporary = result[0];
      result[0] = result[1];
      result[1] = temporary;
    }
    return result;
  }

  if (length === 3) {
    inlineSort3(result, ascending);
    return result;
  }

  // For small-medium arrays, skip integer/range analysis
  if (length <= 128) {
    for (let index = 0; index < length; index++) {
      // biome-ignore lint/suspicious/noSelfCompare: NaN detection
      if (result[index] !== result[index]) {
        return handleNaNSort(result, ascending);
      }
    }
    numericQuickSort(result, 0, length - 1, ascending);
    return result;
  }

  // Check if all numbers are integers and find range.
  // min/max are only needed by the counting-sort branch, which requires
  // allIntegers, so stop tracking them once a non-integer is seen.
  let isAllIntegers = true;
  let min = result[0];
  let max = result[0];
  let hasNaN = false;

  for (let index = 0; index < length; index++) {
    const value = result[index];
    // biome-ignore lint/suspicious/noSelfCompare: ignore
    if (value !== value) {
      hasNaN = true;
      break;
    }
    if (isAllIntegers) {
      if (value !== Math.floor(value)) {
        isAllIntegers = false;
      } else if (value < min) {
        min = value;
      } else if (value > max) {
        max = value;
      }
    }
  }

  // For small integer ranges, use counting sort
  if (
    !hasNaN &&
    isAllIntegers &&
    max - min < length * 2 &&
    max - min < 1_000_000
  ) {
    return countingSort(result, min, max, ascending);
  }

  // Below the crossover, quicksort beats radix sort because of the radix
  // setup and typed-array allocation overhead. The measured crossover
  // depends on the element kind: all-integer input stays faster under
  // quicksort up to roughly twice the size of floating-point input, so it
  // gets a higher threshold.
  const radixThreshold = isAllIntegers ? 2048 : 1024;
  if (length < radixThreshold) {
    if (hasNaN) {
      return handleNaNSort(result, ascending);
    }
    numericQuickSort(result, 0, length - 1, ascending);
    return result;
  }

  // Use IEEE 754 Float64 radix sort for large arrays
  return float64RadixSort(result, ascending, hasNaN);
};

/**
 * IEEE 754 Float64 radix sort
 * Works on all number types (integers and floats) by treating
 * the 64-bit IEEE 754 bit pattern as a sortable unsigned integer.
 *
 * Bit transformation:
 * - Positive numbers (sign bit 0): flip only the sign bit
 * - Negative numbers (sign bit 1): flip ALL 64 bits
 * After this transformation, unsigned integer comparison matches
 * the original floating-point numerical order.
 */
const float64RadixSort = (
  array: number[],
  ascending: boolean,
  hasNaN: boolean,
): number[] => {
  const length = array.length;

  const sourceBuffer = new ArrayBuffer(length * 8);
  const sourceF64 = new Float64Array(sourceBuffer);
  const sourceU32 = new Uint32Array(sourceBuffer);

  // Eight 256-entry histograms (one per byte) packed into a single array.
  // Histogram for pass p occupies the range [p << 8, (p << 8) + 256).
  const histograms = new Uint32Array(256 * 8);

  // Single pass: copy each value into the typed buffer, transform its IEEE 754
  // bit pattern into a sortable unsigned form, and tally all eight byte
  // histograms at once. The byte multiset at each position is invariant under
  // the stable reorderings the radix passes perform, so counting up front is
  // equivalent to counting per pass.
  let validLength: number;

  if (hasNaN) {
    let writeIndex = 0;
    for (let index = 0; index < length; index++) {
      const v = array[index];
      // biome-ignore lint/suspicious/noSelfCompare: NaN detection
      if (v !== v) {
        continue;
      }
      sourceF64[writeIndex] = v;
      const loIndex = writeIndex * 2;
      const hiIndex = loIndex + 1;
      let lo = sourceU32[loIndex];
      let hi = sourceU32[hiIndex];
      if (hi & 0x80_00_00_00) {
        hi = ~hi >>> 0;
        lo = ~lo >>> 0;
      } else {
        hi = (hi ^ 0x80_00_00_00) >>> 0;
      }
      sourceU32[loIndex] = lo;
      sourceU32[hiIndex] = hi;
      histograms[lo & 0xff]++;
      histograms[256 + ((lo >>> 8) & 0xff)]++;
      histograms[512 + ((lo >>> 16) & 0xff)]++;
      histograms[768 + ((lo >>> 24) & 0xff)]++;
      histograms[1024 + (hi & 0xff)]++;
      histograms[1280 + ((hi >>> 8) & 0xff)]++;
      histograms[1536 + ((hi >>> 16) & 0xff)]++;
      histograms[1792 + ((hi >>> 24) & 0xff)]++;
      writeIndex++;
    }
    validLength = writeIndex;
  } else {
    for (let index = 0; index < length; index++) {
      sourceF64[index] = array[index];
      const loIndex = index * 2;
      const hiIndex = loIndex + 1;
      let lo = sourceU32[loIndex];
      let hi = sourceU32[hiIndex];
      if (hi & 0x80_00_00_00) {
        hi = ~hi >>> 0;
        lo = ~lo >>> 0;
      } else {
        hi = (hi ^ 0x80_00_00_00) >>> 0;
      }
      sourceU32[loIndex] = lo;
      sourceU32[hiIndex] = hi;
      histograms[lo & 0xff]++;
      histograms[256 + ((lo >>> 8) & 0xff)]++;
      histograms[512 + ((lo >>> 16) & 0xff)]++;
      histograms[768 + ((lo >>> 24) & 0xff)]++;
      histograms[1024 + (hi & 0xff)]++;
      histograms[1280 + ((hi >>> 8) & 0xff)]++;
      histograms[1536 + ((hi >>> 16) & 0xff)]++;
      histograms[1792 + ((hi >>> 24) & 0xff)]++;
    }
    validLength = length;
  }

  if (validLength === 0) {
    return array;
  }

  const destinationBuffer = new ArrayBuffer(validLength * 8);
  const destinationU32 = new Uint32Array(destinationBuffer);

  let currentSource = sourceU32;
  let currentDestination = destinationU32;
  let isResultInSource = true;

  // 8-pass LSD radix sort (8 bits per pass)
  // Passes 0-3: low 32-bit word (index i*2)
  // Passes 4-7: high 32-bit word (index i*2+1)
  for (let pass = 0; pass < 8; pass++) {
    const base = pass << 8;
    const wordOffset = pass < 4 ? 0 : 1;
    const shift = (pass & 3) << 3;

    // Skip pass if all elements have the same byte value
    let isSkipPass = false;
    for (let index = 0; index < 256; index++) {
      if (histograms[base + index] === validLength) {
        isSkipPass = true;
        break;
      }
    }
    if (isSkipPass) {
      continue;
    }

    // Prefix sum (reverse for descending sort)
    if (ascending) {
      for (let index = 1; index < 256; index++) {
        histograms[base + index] += histograms[base + index - 1];
      }
    } else {
      for (let index = 254; index >= 0; index--) {
        histograms[base + index] += histograms[base + index + 1];
      }
    }

    // Scatter (backward for stability)
    for (let index = validLength - 1; index >= 0; index--) {
      const elementIndex = index * 2;
      const byte = (currentSource[elementIndex + wordOffset] >>> shift) & 0xff;
      const pos = --histograms[base + byte];
      const destinationIndex = pos * 2;
      currentDestination[destinationIndex] = currentSource[elementIndex];
      currentDestination[destinationIndex + 1] =
        currentSource[elementIndex + 1];
    }

    // Ping-pong swap
    const temporary = currentSource;
    currentSource = currentDestination;
    currentDestination = temporary;
    isResultInSource = !isResultInSource;
  }

  // Determine which buffer holds the result
  const resultU32 = currentSource;
  const resultBuffer = isResultInSource ? sourceBuffer : destinationBuffer;
  const resultF64 = new Float64Array(resultBuffer);

  // Reverse the bit transformation and copy each value back in one pass
  for (let index = 0; index < validLength; index++) {
    const loIndex = index * 2;
    const hiIndex = loIndex + 1;
    const hi = resultU32[hiIndex];
    if (hi & 0x80_00_00_00) {
      resultU32[hiIndex] = hi ^ 0x80_00_00_00;
    } else {
      resultU32[hiIndex] = ~hi >>> 0;
      resultU32[loIndex] = ~resultU32[loIndex] >>> 0;
    }
    array[index] = resultF64[index];
  }

  // Append NaN values at the end
  for (let index = validLength; index < length; index++) {
    array[index] = NaN;
  }

  return array;
};

/**
 * Inline sort for 3 elements
 */
const inlineSort3 = (array: number[], ascending: boolean): void => {
  let a = array[0];
  let b = array[1];
  let c = array[2];
  let temporary: number;

  if (ascending) {
    if (a > b) {
      temporary = a;
      a = b;
      b = temporary;
    }
    if (b > c) {
      temporary = b;
      b = c;
      c = temporary;
      if (a > b) {
        temporary = a;
        a = b;
        b = temporary;
      }
    }
  } else {
    if (a < b) {
      temporary = a;
      a = b;
      b = temporary;
    }
    if (b < c) {
      temporary = b;
      b = c;
      c = temporary;
      if (a < b) {
        temporary = a;
        a = b;
        b = temporary;
      }
    }
  }

  array[0] = a;
  array[1] = b;
  array[2] = c;
};

/**
 * Handle arrays with NaN values
 */
const handleNaNSort = (array: number[], ascending: boolean): number[] => {
  const valid: number[] = [];
  let nanCount = 0;

  let index = 0;
  while (index < array.length) {
    const element = array[index];
    // biome-ignore lint/suspicious/noSelfCompare: ignore
    if (element === element) {
      valid.push(element);
    } else {
      nanCount++;
    }
    index++;
  }

  numericQuickSort(valid, 0, valid.length - 1, ascending);

  // NaN values go to the end
  for (let index = 0; index < nanCount; index++) {
    valid.push(NaN);
  }

  // Copy back
  for (let index = 0; index < array.length; index++) {
    array[index] = valid[index];
  }

  return array;
};

/**
 * Counting sort for small integer ranges
 */
const countingSort = (
  array: number[],
  min: number,
  max: number,
  ascending: boolean,
): number[] => {
  const range = max - min + 1;
  const count = new Uint32Array(range);

  // Count occurrences
  let index_ = 0;
  while (index_ < array.length) {
    count[array[index_] - min]++;
    index_++;
  }

  // Reconstruct array
  let index = 0;
  if (ascending) {
    for (let index_ = 0; index_ < range; index_++) {
      const cnt = count[index_];
      const value = index_ + min;
      for (let index__ = 0; index__ < cnt; index__++) {
        array[index++] = value;
      }
    }
  } else {
    for (let index_ = range - 1; index_ >= 0; index_--) {
      const cnt = count[index_];
      const value = index_ + min;
      for (let index__ = 0; index__ < cnt; index__++) {
        array[index++] = value;
      }
    }
  }

  return array;
};

/**
 * Optimized quicksort for numbers
 */
const numericQuickSort = (
  array: number[],
  low: number,
  high: number,
  ascending: boolean,
): number[] => {
  const stack: number[] = [low, high];

  while (stack.length > 0) {
    const h = stack.pop();
    const l = stack.pop();

    if (h === undefined || l === undefined || h <= l) {
      continue;
    }

    // For small subarrays, use insertion sort
    if (h - l < 16) {
      numericInsertionSort(array, l, h, ascending);
      continue;
    }

    // Partition
    const pivot = numericPartition(array, l, h, ascending);

    // Push larger partition first to limit stack depth
    if (pivot - l > h - pivot) {
      stack.push(l, pivot - 1, pivot + 1, h);
    } else {
      stack.push(pivot + 1, h, l, pivot - 1);
    }
  }

  return array;
};

/**
 * Numeric insertion sort
 */
const numericInsertionSort = (
  array: number[],
  low: number,
  high: number,
  ascending: boolean,
): void => {
  if (ascending) {
    for (let index = low + 1; index <= high; index++) {
      const key = array[index];
      let index_ = index - 1;
      while (index_ >= low && array[index_] > key) {
        array[index_ + 1] = array[index_];
        index_--;
      }
      array[index_ + 1] = key;
    }
  } else {
    for (let index = low + 1; index <= high; index++) {
      const key = array[index];
      let index_ = index - 1;
      while (index_ >= low && array[index_] < key) {
        array[index_ + 1] = array[index_];
        index_--;
      }
      array[index_ + 1] = key;
    }
  }
};

/**
 * Numeric partition with median-of-three pivot
 */
const numericPartition = (
  array: number[],
  low: number,
  high: number,
  ascending: boolean,
): number => {
  // Median-of-three pivot selection
  const mid = low + ((high - low) >> 1);
  let temporary: number;

  if (ascending) {
    if (array[mid] < array[low]) {
      temporary = array[low];
      array[low] = array[mid];
      array[mid] = temporary;
    }
    if (array[high] < array[low]) {
      temporary = array[low];
      array[low] = array[high];
      array[high] = temporary;
    }
    if (array[high] < array[mid]) {
      temporary = array[mid];
      array[mid] = array[high];
      array[high] = temporary;
    }
  } else {
    if (array[mid] > array[low]) {
      temporary = array[low];
      array[low] = array[mid];
      array[mid] = temporary;
    }
    if (array[high] > array[low]) {
      temporary = array[low];
      array[low] = array[high];
      array[high] = temporary;
    }
    if (array[high] > array[mid]) {
      temporary = array[mid];
      array[mid] = array[high];
      array[high] = temporary;
    }
  }

  // Move pivot to end-1
  temporary = array[mid];
  array[mid] = array[high - 1];
  array[high - 1] = temporary;
  const pivot = array[high - 1];

  let index = low;
  let index_ = high - 1;

  if (ascending) {
    while (true) {
      while (array[++index] < pivot) {
        // Continue
      }
      while (array[--index_] > pivot) {
        // Continue
      }
      if (index >= index_) {
        break;
      }
      temporary = array[index];
      array[index] = array[index_];
      array[index_] = temporary;
    }
  } else {
    while (true) {
      while (array[++index] > pivot) {
        // Continue
      }
      while (array[--index_] < pivot) {
        // Continue
      }
      if (index >= index_) {
        break;
      }
      temporary = array[index];
      array[index] = array[index_];
      array[index_] = temporary;
    }
  }

  temporary = array[index];
  array[index] = array[high - 1];
  array[high - 1] = temporary;
  return index;
};
