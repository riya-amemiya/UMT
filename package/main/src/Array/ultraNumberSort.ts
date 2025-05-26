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
    return array;
  }

  // For tiny arrays, use optimized inline sort
  if (length === 2) {
    if (array[0] > array[1] === ascending) {
      [array[0], array[1]] = [array[1], array[0]];
    }
    return array;
  }

  if (length === 3) {
    inlineSort3(array, ascending);
    return array;
  }

  // Check if all numbers are integers and find range
  let allIntegers = true;
  let min = array[0];
  let max = array[0];
  let hasNaN = false;

  for (let index = 0; index < length; index++) {
    const value = array[index];
    if (Number.isNaN(value)) {
      hasNaN = true;
      break;
    }
    if (value < min) {
      min = value;
    }
    if (value > max) {
      max = value;
    }
    if (allIntegers && value !== Math.floor(value)) {
      allIntegers = false;
    }
  }

  // Handle NaN values
  if (hasNaN) {
    return handleNaNSort(array, ascending);
  }

  // For small integer ranges, use counting sort
  if (allIntegers && max - min < length * 2 && max - min < 1_000_000) {
    return countingSort(array, min, max, ascending);
  }

  // For larger arrays, use radix sort if applicable
  if (allIntegers && length > 100) {
    return radixSort(array, ascending);
  }

  // Fall back to optimized quicksort for floating point
  return numericQuickSort(array, 0, length - 1, ascending);
};

/**
 * Inline sort for 3 elements
 */
const inlineSort3 = (array: number[], ascending: boolean): void => {
  let a = array[0];
  let b = array[1];
  let c = array[2];

  if (ascending) {
    if (a > b) {
      [a, b] = [b, a];
    }
    if (b > c) {
      [b, c] = [c, b];
      if (a > b) {
        [a, b] = [b, a];
      }
    }
  } else {
    if (a < b) {
      [a, b] = [b, a];
    }
    if (b < c) {
      [b, c] = [c, b];
      if (a < b) {
        [a, b] = [b, a];
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

  for (const element of array) {
    if (Number.isNaN(element)) {
      nanCount++;
    } else {
      valid.push(element);
    }
  }

  numericQuickSort(valid, 0, valid.length - 1, ascending);

  // NaN values go to the end
  for (let index = 0; index < nanCount; index++) {
    valid.push(Number.NaN);
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
  for (const element of array) {
    count[element - min]++;
  }

  // Reconstruct array
  let index = 0;
  if (ascending) {
    for (let index_ = 0; index_ < range; index_++) {
      const cnt = count[index_];
      const value = index_ + min;
      for (let index_ = 0; index_ < cnt; index_++) {
        array[index++] = value;
      }
    }
  } else {
    for (let index_ = range - 1; index_ >= 0; index_--) {
      const cnt = count[index_];
      const value = index_ + min;
      for (let index_ = 0; index_ < cnt; index_++) {
        array[index++] = value;
      }
    }
  }

  return array;
};

/**
 * Radix sort for integers
 */
const radixSort = (array: number[], ascending: boolean): number[] => {
  const length = array.length;

  // Separate positive and negative numbers
  const positive: number[] = [];
  const negative: number[] = [];
  let zeroCount = 0;

  for (let index_ = 0; index_ < length; index_++) {
    if (array[index_] > 0) {
      positive.push(array[index_]);
    } else if (array[index_] < 0) {
      negative.push(-array[index_]);
    } else {
      zeroCount++;
    }
  }

  // Sort positive numbers
  if (positive.length > 0) {
    radixSortPositive(positive);
  }

  // Sort negative numbers
  if (negative.length > 0) {
    radixSortPositive(negative);
  }

  // Merge results
  let index = 0;
  if (ascending) {
    // Negative numbers first (in reverse order)
    for (let index_ = negative.length - 1; index_ >= 0; index_--) {
      array[index++] = -negative[index_];
    }
    // Zeros
    for (let index_ = 0; index_ < zeroCount; index_++) {
      array[index++] = 0;
    }
    // Positive numbers
    for (const element of positive) {
      array[index++] = element;
    }
  } else {
    // Positive numbers first (in reverse order)
    for (let index_ = positive.length - 1; index_ >= 0; index_--) {
      array[index++] = positive[index_];
    }
    // Zeros
    for (let index_ = 0; index_ < zeroCount; index_++) {
      array[index++] = 0;
    }
    // Negative numbers
    for (const element of negative) {
      array[index++] = -element;
    }
  }

  return array;
};

/**
 * Radix sort for positive integers
 */
const radixSortPositive = (array: number[]): void => {
  const length = array.length;
  if (length <= 1) {
    return;
  }

  // Find maximum to determine number of digits
  let max = array[0];
  for (let index = 1; index < length; index++) {
    if (array[index] > max) {
      max = array[index];
    }
  }

  // Use typed arrays for better performance
  const output = new Float64Array(length);
  const count = new Uint32Array(256);

  // Process 8 bits at a time
  for (let shift = 0; max >> shift > 0; shift += 8) {
    // Reset count array
    count.fill(0);

    // Count occurrences
    for (let index = 0; index < length; index++) {
      const digit = (array[index] >> shift) & 0xff;
      count[digit]++;
    }

    // Change count[i] to actual position
    for (let index = 1; index < 256; index++) {
      count[index] += count[index - 1];
    }

    // Build output array
    for (let index = length - 1; index >= 0; index--) {
      const digit = (array[index] >> shift) & 0xff;
      output[--count[digit]] = array[index];
    }

    // Copy back
    for (let index = 0; index < length; index++) {
      array[index] = output[index];
    }
  }
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
  const stack: number[] = [];
  stack.push(low, high);

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

  if (ascending) {
    if (array[mid] < array[low]) {
      [array[low], array[mid]] = [array[mid], array[low]];
    }
    if (array[high] < array[low]) {
      [array[low], array[high]] = [array[high], array[low]];
    }
    if (array[high] < array[mid]) {
      [array[mid], array[high]] = [array[high], array[mid]];
    }
  } else {
    if (array[mid] > array[low]) {
      [array[low], array[mid]] = [array[mid], array[low]];
    }
    if (array[high] > array[low]) {
      [array[low], array[high]] = [array[high], array[low]];
    }
    if (array[high] > array[mid]) {
      [array[mid], array[high]] = [array[high], array[mid]];
    }
  }

  // Move pivot to end-1
  [array[mid], array[high - 1]] = [array[high - 1], array[mid]];
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
      [array[index], array[index_]] = [array[index_], array[index]];
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
      [array[index], array[index_]] = [array[index_], array[index]];
    }
  }

  [array[index], array[high - 1]] = [array[high - 1], array[index]];
  return index;
};
