import { networkSort } from "@/Array/sortingHelpers/networkSort";
import { compareFunctionDefault } from "@/Array/compareFunctionDefault";

describe("networkSort", () => {
  describe("ascending order", () => {
    it("should sort an array of 2 elements", () => {
      const arr1 = [2, 1];
      networkSort(arr1, 0, 1, compareFunctionDefault);
      expect(arr1).toEqual([1, 2]);

      const arr2 = [1, 2];
      networkSort(arr2, 0, 1, compareFunctionDefault);
      expect(arr2).toEqual([1, 2]);
    });

    it("should sort an array of 3 elements", () => {
      const arr1 = [3, 1, 2];
      networkSort(arr1, 0, 2, compareFunctionDefault);
      expect(arr1).toEqual([1, 2, 3]);

      const arr2 = [1, 3, 2];
      networkSort(arr2, 0, 2, compareFunctionDefault);
      expect(arr2).toEqual([1, 2, 3]);

      const arr3 = [2, 3, 1];
      networkSort(arr3, 0, 2, compareFunctionDefault);
      expect(arr3).toEqual([1, 2, 3]);
    });

    it("should sort an array of 4 elements", () => {
      const arr1 = [4, 1, 3, 2];
      networkSort(arr1, 0, 3, compareFunctionDefault);
      expect(arr1).toEqual([1, 2, 3, 4]);

      const arr2 = [2, 4, 1, 3];
      networkSort(arr2, 0, 3, compareFunctionDefault);
      expect(arr2).toEqual([1, 2, 3, 4]);
    });

    it("should sort an array of 5 elements", () => {
      const arr1 = [5, 1, 4, 2, 3];
      networkSort(arr1, 0, 4, compareFunctionDefault);
      expect(arr1).toEqual([1, 2, 3, 4, 5]);

      const arr2 = [3, 5, 1, 4, 2];
      networkSort(arr2, 0, 4, compareFunctionDefault);
      expect(arr2).toEqual([1, 2, 3, 4, 5]);

      const arr3 = [5, 4, 3, 2, 1];
      networkSort(arr3, 0, 4, compareFunctionDefault);
      expect(arr3).toEqual([1, 2, 3, 4, 5]);

      const arr4 = [1, 2, 5, 4, 3];
      networkSort(arr4, 0, 4, compareFunctionDefault);
      expect(arr4).toEqual([1, 2, 3, 4, 5]);
    });
  });

  describe("descending order", () => {
    const descendingCompare = (a: number, b: number) => b - a;
    it("should sort an array of 2 elements in descending order", () => {
      const arr = [1, 2];
      networkSort(arr, 0, 1, descendingCompare);
      expect(arr).toEqual([2, 1]);
    });

    it("should sort an array of 3 elements in descending order", () => {
      const arr = [1, 3, 2];
      networkSort(arr, 0, 2, descendingCompare);
      expect(arr).toEqual([3, 2, 1]);
    });

    it("should sort an array of 4 elements in descending order", () => {
      const arr = [1, 4, 2, 3];
      networkSort(arr, 0, 3, descendingCompare);
      expect(arr).toEqual([4, 3, 2, 1]);
    });

    it("should sort an array of 5 elements in descending order", () => {
      const arr = [1, 5, 2, 4, 3];
      networkSort(arr, 0, 4, descendingCompare);
      expect(arr).toEqual([5, 4, 3, 2, 1]);
    });
  });

  describe("sub-array sorting", () => {
    it("should sort a sub-array of 3 elements", () => {
      const arr = [0, 3, 1, 2, 4];
      networkSort(arr, 1, 3, compareFunctionDefault);
      expect(arr).toEqual([0, 1, 2, 3, 4]);
    });

    it("should sort a sub-array of 5 elements", () => {
      const arr = [9, 5, 1, 4, 2, 3, 0];
      networkSort(arr, 1, 5, compareFunctionDefault);
      expect(arr).toEqual([9, 1, 2, 3, 4, 5, 0]);
    });
  });

  describe("arrays with length outside 2-5", () => {
    it("should not modify an array with 1 element", () => {
      const arr = [1];
      const originalArr = [...arr];
      networkSort(arr, 0, 0, compareFunctionDefault);
      expect(arr).toEqual(originalArr);
    });

    it("should not modify an empty array", () => {
      const arr: number[] = [];
      const originalArr = [...arr];
      networkSort(arr, 0, -1, compareFunctionDefault);
      expect(arr).toEqual(originalArr);
    });

    it("should not modify an array with 0 elements (low > high)", () => {
      const arr = [1, 2, 3];
      const originalArr = [...arr];
      networkSort(arr, 1, 0, compareFunctionDefault);
      expect(arr).toEqual(originalArr);
    });

    it("should not modify an array with more than 5 elements if range is > 5", () => {
      const arr = [6, 5, 4, 3, 2, 1];
      const originalArr = [...arr];
      networkSort(arr, 0, 5, compareFunctionDefault);
      expect(arr).toEqual(originalArr);
    });

    it("should sort a sub-array of 2 elements within a larger array", () => {
      const arr = [0, 9, 8, 7, 6, 5];
      networkSort(arr, 1, 2, compareFunctionDefault); // sort [9,8]
      expect(arr).toEqual([0, 8, 9, 7, 6, 5]);
    });
  });
});
