import { ultraNumberSort } from "@/Array/ultraNumberSort";

describe("ultraNumberSort", () => {
  it("should sort an array of numbers in ascending order by default", () => {
    const arr = [3, 1, 4, 1, 5, 9, 2, 6];
    expect(ultraNumberSort([...arr])).toEqual([1, 1, 2, 3, 4, 5, 6, 9]);
  });

  it("should sort an array of numbers in ascending order when specified", () => {
    const arr = [3, 1, 4, 1, 5, 9, 2, 6];
    expect(ultraNumberSort([...arr], true)).toEqual([1, 1, 2, 3, 4, 5, 6, 9]);
  });

  it("should sort an array of numbers in descending order when specified", () => {
    const arr = [3, 1, 4, 1, 5, 9, 2, 6];
    expect(ultraNumberSort([...arr], false)).toEqual([9, 6, 5, 4, 3, 2, 1, 1]);
  });

  it("should return an empty array when given an empty array", () => {
    expect(ultraNumberSort([])).toEqual([]);
    expect(ultraNumberSort([], false)).toEqual([]);
  });

  it("should return the same array when given an array with one element", () => {
    expect(ultraNumberSort([1])).toEqual([1]);
    expect(ultraNumberSort([1], false)).toEqual([1]);
  });

  it("should sort an array with two elements in ascending order", () => {
    expect(ultraNumberSort([2, 1])).toEqual([1, 2]);
    expect(ultraNumberSort([1, 2])).toEqual([1, 2]);
  });

  it("should sort an array with two elements in descending order", () => {
    expect(ultraNumberSort([2, 1], false)).toEqual([2, 1]);
    expect(ultraNumberSort([1, 2], false)).toEqual([2, 1]);
  });

  it("should sort an array with three elements in ascending order", () => {
    expect(ultraNumberSort([3, 1, 2])).toEqual([1, 2, 3]);
    expect(ultraNumberSort([1, 3, 2])).toEqual([1, 2, 3]);
    expect(ultraNumberSort([2, 1, 3])).toEqual([1, 2, 3]);
  });

  it("should sort an array with three elements in descending order", () => {
    expect(ultraNumberSort([3, 1, 2], false)).toEqual([3, 2, 1]);
    expect(ultraNumberSort([1, 3, 2], false)).toEqual([3, 2, 1]);
    expect(ultraNumberSort([2, 1, 3], false)).toEqual([3, 2, 1]);
  });

  it("should handle arrays with duplicate values", () => {
    const arr = [5, 2, 8, 2, 5, 8, 2];
    expect(ultraNumberSort([...arr])).toEqual([2, 2, 2, 5, 5, 8, 8]);
    expect(ultraNumberSort([...arr], false)).toEqual([8, 8, 5, 5, 2, 2, 2]);
  });

  it("should handle arrays with negative numbers", () => {
    const arr = [-3, 1, -4, 0, 5, -9, 2, -6];
    expect(ultraNumberSort([...arr])).toEqual([-9, -6, -4, -3, 0, 1, 2, 5]);
    expect(ultraNumberSort([...arr], false)).toEqual([
      5, 2, 1, 0, -3, -4, -6, -9,
    ]);
  });

  it("should handle arrays with floating-point numbers", () => {
    const arr = [3.14, 1.0, 4.5, 1.0, 5.9, 9.2, 2.6, 6.0];
    expect(ultraNumberSort([...arr])).toEqual([
      1.0, 1.0, 2.6, 3.14, 4.5, 5.9, 6.0, 9.2,
    ]);
    expect(ultraNumberSort([...arr], false)).toEqual([
      9.2, 6.0, 5.9, 4.5, 3.14, 2.6, 1.0, 1.0,
    ]);
  });

  it("should handle arrays with mixed integers and floating-point numbers", () => {
    const arr = [3, 1.0, 4.5, 1, 5.9, 9, 2.6, 6];
    expect(ultraNumberSort([...arr])).toEqual([1, 1.0, 2.6, 3, 4.5, 5.9, 6, 9]);
    expect(ultraNumberSort([...arr], false)).toEqual([
      9, 6, 5.9, 4.5, 3, 2.6, 1.0, 1,
    ]);
  });

  it("should handle arrays that are already sorted", () => {
    const arr = [1, 2, 3, 4, 5];
    expect(ultraNumberSort([...arr])).toEqual([1, 2, 3, 4, 5]);
    expect(ultraNumberSort([...arr], false)).toEqual([5, 4, 3, 2, 1]);
  });

  it("should handle arrays that are reverse sorted", () => {
    const arr = [5, 4, 3, 2, 1];
    expect(ultraNumberSort([...arr])).toEqual([1, 2, 3, 4, 5]);
    expect(ultraNumberSort([...arr], false)).toEqual([5, 4, 3, 2, 1]);
  });

  it("should handle arrays with NaN values", () => {
    const arr = [3, Number.NaN, 1, 4, Number.NaN, 1, 5, 9, 2, 6];
    const sortedAsc = ultraNumberSort([...arr]);
    expect(sortedAsc.slice(0, -2)).toEqual([1, 1, 2, 3, 4, 5, 6, 9]);
    expect(Number.isNaN(sortedAsc[sortedAsc.length - 2])).toBe(true);
    expect(Number.isNaN(sortedAsc[sortedAsc.length - 1])).toBe(true);

    const sortedDesc = ultraNumberSort([...arr], false);
    expect(sortedDesc.slice(0, -2)).toEqual([9, 6, 5, 4, 3, 2, 1, 1]);
    expect(Number.isNaN(sortedDesc[sortedDesc.length - 2])).toBe(true);
    expect(Number.isNaN(sortedDesc[sortedDesc.length - 1])).toBe(true);
  });

  it("should handle arrays with only NaN values", () => {
    const arr = [Number.NaN, Number.NaN, Number.NaN];
    const sorted = ultraNumberSort([...arr]);
    expect(sorted.every(Number.isNaN)).toBe(true);
    expect(sorted.length).toBe(3);
  });

  it("should handle arrays with Infinity and -Infinity", () => {
    const arr = [
      Number.POSITIVE_INFINITY,
      Number.NEGATIVE_INFINITY,
      0,
      100,
      -100,
    ];
    expect(ultraNumberSort([...arr])).toEqual([
      Number.NEGATIVE_INFINITY,
      -100,
      0,
      100,
      Number.POSITIVE_INFINITY,
    ]);
    expect(ultraNumberSort([...arr], false)).toEqual([
      Number.POSITIVE_INFINITY,
      100,
      0,
      -100,
      Number.NEGATIVE_INFINITY,
    ]);
  });

  it("should use counting sort for small integer ranges (ascending)", () => {
    const arr = Array.from(
      { length: 50 },
      (_) => Math.floor(Math.random() * 10) + 1,
    );
    const sortedArr = [...arr].sort((a, b) => a - b);
    expect(ultraNumberSort([...arr])).toEqual(sortedArr);
  });

  it("should use counting sort for small integer ranges (descending)", () => {
    const arr = Array.from(
      { length: 50 },
      (_) => Math.floor(Math.random() * 10) + 1,
    );
    const sortedArr = [...arr].sort((a, b) => b - a);
    expect(ultraNumberSort([...arr], false)).toEqual(sortedArr);
  });

  it("should use radix sort for larger integer arrays (ascending)", () => {
    const arr = Array.from(
      { length: 200 },
      (_) => Math.floor(Math.random() * 1000) - 500,
    );
    const sortedArr = [...arr].sort((a, b) => a - b);
    expect(ultraNumberSort([...arr])).toEqual(sortedArr);
  });

  it("should use radix sort for larger integer arrays (descending)", () => {
    const arr = Array.from(
      { length: 200 },
      (_) => Math.floor(Math.random() * 1000) - 500,
    );
    const sortedArr = [...arr].sort((a, b) => b - a);
    expect(ultraNumberSort([...arr], false)).toEqual(sortedArr);
  });

  it("should correctly sort a large array of random numbers (ascending)", () => {
    const arr = Array.from(
      { length: 1000 },
      (_) => Math.random() * 2000 - 1000,
    );
    const sortedArr = [...arr].sort((a, b) => a - b);
    expect(ultraNumberSort([...arr])).toEqual(sortedArr);
  });

  it("should correctly sort a large array of random numbers (descending)", () => {
    const arr = Array.from({ length: 1000 }, () => Math.random() * 2000 - 1000);
    const sortedArr = [...arr].sort((a, b) => b - a);
    expect(ultraNumberSort([...arr], false)).toEqual(sortedArr);
  });

  it("should handle arrays with all same elements", () => {
    const arr = [7, 7, 7, 7, 7];
    expect(ultraNumberSort([...arr])).toEqual([7, 7, 7, 7, 7]);
    expect(ultraNumberSort([...arr], false)).toEqual([7, 7, 7, 7, 7]);
  });

  it("should handle radixSort with only positive numbers", () => {
    const arr = Array.from(
      { length: 150 },
      () => Math.floor(Math.random() * 500) + 1,
    );
    const sortedArr = [...arr].sort((a, b) => a - b);
    expect(ultraNumberSort([...arr])).toEqual(sortedArr);
  });

  it("should handle radixSort with only negative numbers", () => {
    const arr = Array.from(
      { length: 150 },
      () => Math.floor(Math.random() * -500) - 1,
    );
    const sortedArr = [...arr].sort((a, b) => a - b);
    expect(ultraNumberSort([...arr])).toEqual(sortedArr);
  });

  it("should handle radixSort with only zeros and length > 100", () => {
    const arr = Array(101).fill(0);
    expect(ultraNumberSort([...arr])).toEqual(arr);
    expect(ultraNumberSort([...arr], false)).toEqual(arr);
  });

  it("should handle radixSortPositive with single element array via ultraNumberSort", () => {
    const arrWithOnePositive = Array(101).fill(0).concat([5]);
    expect(ultraNumberSort([...arrWithOnePositive])).toEqual(
      Array(101).fill(0).concat([5]),
    );
    const arrWithOneNegative = Array(101).fill(0).concat([-5]);
    expect(ultraNumberSort([...arrWithOneNegative])).toEqual(
      [-5].concat(Array(101).fill(0)),
    );
  });

  it("should correctly sort all permutations of 3 elements ascending", () => {
    const permutations = [
      [1, 2, 3],
      [1, 3, 2],
      [2, 1, 3],
      [2, 3, 1],
      [3, 1, 2],
      [3, 2, 1],
    ];
    const expected = [1, 2, 3];
    for (const p of permutations) {
      expect(ultraNumberSort([...p], true)).toEqual(expected);
    }
  });

  it("should correctly sort all permutations of 3 elements descending", () => {
    const permutations = [
      [1, 2, 3],
      [1, 3, 2],
      [2, 1, 3],
      [2, 3, 1],
      [3, 1, 2],
      [3, 2, 1],
    ];
    const expected = [3, 2, 1];
    for (const p of permutations) {
      expect(ultraNumberSort([...p], false)).toEqual(expected);
    }
  });

  it("should cover radixSort branches with mixed positive, negative, and zeros", () => {
    const arr1 = Array.from(
      { length: 60 },
      () => Math.floor(Math.random() * 100) + 1,
    ).concat(
      Array.from({ length: 60 }, () => Math.floor(Math.random() * -100) - 1),
    );
    const sortedArr1Asc = [...arr1].sort((a, b) => a - b);
    const sortedArr1Desc = [...arr1].sort((a, b) => b - a);
    expect(ultraNumberSort([...arr1])).toEqual(sortedArr1Asc);
    expect(ultraNumberSort([...arr1], false)).toEqual(sortedArr1Desc);

    const arr2 = Array.from(
      { length: 60 },
      () => Math.floor(Math.random() * 100) + 1,
    ).concat(Array(60).fill(0));
    const sortedArr2Asc = [...arr2].sort((a, b) => a - b);
    const sortedArr2Desc = [...arr2].sort((a, b) => b - a);
    expect(ultraNumberSort([...arr2])).toEqual(sortedArr2Asc);
    expect(ultraNumberSort([...arr2], false)).toEqual(sortedArr2Desc);

    const arr3 = Array.from(
      { length: 60 },
      () => Math.floor(Math.random() * -100) - 1,
    ).concat(Array(60).fill(0));
    const sortedArr3Asc = [...arr3].sort((a, b) => a - b);
    const sortedArr3Desc = [...arr3].sort((a, b) => b - a);
    expect(ultraNumberSort([...arr3])).toEqual(sortedArr3Asc);
    expect(ultraNumberSort([...arr3], false)).toEqual(sortedArr3Desc);
  });

  it("should cover specific radixSort and radixSortPositive branches for 100% coverage", () => {
    const makeArr = (len: number, valFn: (i: number) => number) =>
      Array.from({ length: len }, (_, i) => valFn(i));

    const arrWithZerosAndLargeRangeForRadix = [
      0,
      1000000,
      ...makeArr(99, () => 0),
    ];
    const sortedArrWithZerosAndLargeRangeForRadixAsc = [
      ...arrWithZerosAndLargeRangeForRadix,
    ].sort((a, b) => a - b);
    expect(ultraNumberSort([...arrWithZerosAndLargeRangeForRadix])).toEqual(
      sortedArrWithZerosAndLargeRangeForRadixAsc,
    );
    const sortedArrWithZerosAndLargeRangeForRadixDesc = [
      ...arrWithZerosAndLargeRangeForRadix,
    ].sort((a, b) => b - a);
    expect(
      ultraNumberSort([...arrWithZerosAndLargeRangeForRadix], false),
    ).toEqual(sortedArrWithZerosAndLargeRangeForRadixDesc);

    const arrPositiveOneZeroMany = [500000, ...makeArr(100, () => 0)];
    const sortedArrPositiveOneZeroManyAsc = [...arrPositiveOneZeroMany].sort(
      (a, b) => a - b,
    );
    expect(ultraNumberSort([...arrPositiveOneZeroMany])).toEqual(
      sortedArrPositiveOneZeroManyAsc,
    );
    const sortedArrPositiveOneZeroManyDesc = [...arrPositiveOneZeroMany].sort(
      (a, b) => b - a,
    );
    expect(ultraNumberSort([...arrPositiveOneZeroMany], false)).toEqual(
      sortedArrPositiveOneZeroManyDesc,
    );

    const arrNegativeOneZeroMany = [-500000, ...makeArr(100, () => 0)];
    const sortedArrNegativeOneZeroManyAsc = [...arrNegativeOneZeroMany].sort(
      (a, b) => a - b,
    );
    expect(ultraNumberSort([...arrNegativeOneZeroMany])).toEqual(
      sortedArrNegativeOneZeroManyAsc,
    );
    const sortedArrNegativeOneZeroManyDesc = [...arrNegativeOneZeroMany].sort(
      (a, b) => b - a,
    );
    expect(ultraNumberSort([...arrNegativeOneZeroMany], false)).toEqual(
      sortedArrNegativeOneZeroManyDesc,
    );

    const arrNoZerosTriggerRadix = [
      ...makeArr(51, (i) => 200000 + i),
      ...makeArr(50, (i) => -(200000 + i)),
    ];
    const sortedArrNoZerosTriggerRadixAsc = [...arrNoZerosTriggerRadix].sort(
      (a, b) => a - b,
    );
    expect(ultraNumberSort([...arrNoZerosTriggerRadix])).toEqual(
      sortedArrNoZerosTriggerRadixAsc,
    );
    const sortedArrNoZerosTriggerRadixDesc = [...arrNoZerosTriggerRadix].sort(
      (a, b) => b - a,
    );
    expect(ultraNumberSort([...arrNoZerosTriggerRadix], false)).toEqual(
      sortedArrNoZerosTriggerRadixDesc,
    );

    const arrOnlyPositiveTriggerRadix = makeArr(101, (i) => 200000 + i);
    const sortedArrOnlyPositiveTriggerRadixAsc = [
      ...arrOnlyPositiveTriggerRadix,
    ].sort((a, b) => a - b);
    expect(ultraNumberSort([...arrOnlyPositiveTriggerRadix])).toEqual(
      sortedArrOnlyPositiveTriggerRadixAsc,
    );

    const arrOnlyNegativeTriggerRadix = makeArr(101, (i) => -(200000 + i));
    const sortedArrOnlyNegativeTriggerRadixAsc = [
      ...arrOnlyNegativeTriggerRadix,
    ].sort((a, b) => a - b);
    expect(ultraNumberSort([...arrOnlyNegativeTriggerRadix])).toEqual(
      sortedArrOnlyNegativeTriggerRadixAsc,
    );

    const arrPositiveEmptyTriggerRadix = [...makeArr(101, () => -200000)];
    const sortedArrPositiveEmptyTriggerRadixAsc = [
      ...arrPositiveEmptyTriggerRadix,
    ].sort((a, b) => a - b);
    expect(ultraNumberSort([...arrPositiveEmptyTriggerRadix])).toEqual(
      sortedArrPositiveEmptyTriggerRadixAsc,
    );

    const arrNegativeEmptyTriggerRadix = [...makeArr(101, () => 200000)];
    const sortedArrNegativeEmptyTriggerRadixAsc = [
      ...arrNegativeEmptyTriggerRadix,
    ].sort((a, b) => a - b);
    expect(ultraNumberSort([...arrNegativeEmptyTriggerRadix])).toEqual(
      sortedArrNegativeEmptyTriggerRadixAsc,
    );
  });
});
