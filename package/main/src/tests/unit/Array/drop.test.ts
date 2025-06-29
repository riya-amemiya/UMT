import { drop } from "@/Array/drop";
import _ from "lodash";

describe("drop function", () => {
  it("should exclude n elements from the start of the array", () => {
    // umt
    expect(drop([1, 2, 3, 4, 5], 2)).toEqual([3, 4, 5]);
    expect(drop([1, 2, 3, 4, 5])).toEqual([2, 3, 4, 5]);

    // lodash
    expect(_.drop([1, 2, 3, 4, 5], 2)).toEqual([3, 4, 5]);
    expect(_.drop([1, 2, 3, 4, 5])).toEqual([2, 3, 4, 5]);
  });

  it("should exclude n elements from the start when direction is 'left'", () => {
    // umt
    expect(drop([1, 2, 3, 4, 5], 2, "left")).toEqual([3, 4, 5]);

    // lodash
    expect(_.drop([1, 2, 3, 4, 5], 2)).toEqual([3, 4, 5]);
  });

  it("should exclude n elements from the end when direction is 'right'", () => {
    // umt
    expect(drop([1, 2, 3, 4, 5], 2, "right")).toEqual([1, 2, 3]);

    // lodash
    expect(_.dropRight([1, 2, 3, 4, 5], 2)).toEqual([1, 2, 3]);
  });

  it("should exclude n elements from the middle when direction is 'between'", () => {
    // umt
    expect(drop([1, 2, 3, 4, 5], 1, "between")).toEqual([1, 2, 4, 5]);
    expect(drop([1, 2, 3, 4, 5, 6], 2, "between")).toEqual([1, 2, 5, 6]);
  });

  it("should return an empty array when n is greater than or equal to array length", () => {
    // umt
    expect(drop([1, 2, 3], 4)).toEqual([]);
    expect(drop([1, 2, 3], 3)).toEqual([]);

    // lodash
    expect(_.drop([1, 2, 3], 4)).toEqual([]);
    expect(_.drop([1, 2, 3], 3)).toEqual([]);
  });

  it("should return the original array when n is 0", () => {
    // umt
    expect(drop([1, 2, 3], 0)).toEqual([1, 2, 3]);
    expect(drop([1, 2, 3], 0, "left")).toEqual([1, 2, 3]);
    expect(drop([1, 2, 3], 0, "right")).toEqual([1, 2, 3]);
    expect(drop([1, 2, 3], 0, "between")).toEqual([1, 2, 3]);

    // lodash
    expect(_.drop([1, 2, 3], 0)).toEqual([1, 2, 3]);
    expect(_.drop([1, 2, 3], 0)).toEqual([1, 2, 3]);
    expect(_.dropRight([1, 2, 3], 0)).toEqual([1, 2, 3]);
  });

  it("should return the original array when n is negative", () => {
    // umt
    expect(drop([1, 2, 3], -1)).toEqual([1, 2, 3]);
    expect(drop([1, 2, 3], -2, "left")).toEqual([1, 2, 3]);
    expect(drop([1, 2, 3], -3, "right")).toEqual([1, 2, 3]);
    expect(drop([1, 2, 3], -4, "between")).toEqual([1, 2, 3]);

    // lodash
    expect(_.drop([1, 2, 3], -1)).toEqual([1, 2, 3]);
    expect(_.drop([1, 2, 3], -2)).toEqual([1, 2, 3]);
    expect(_.dropRight([1, 2, 3], -3)).toEqual([1, 2, 3]);
  });

  it("should return an empty array when an empty array is passed", () => {
    // umt
    expect(drop([], 1)).toEqual([]);
    expect(drop([], 2, "left")).toEqual([]);
    expect(drop([], 3, "right")).toEqual([]);
    expect(drop([], 4, "between")).toEqual([]);

    // lodash
    expect(_.drop([], 1)).toEqual([]);
    expect(_.drop([], 2)).toEqual([]);
    expect(_.dropRight([], 3)).toEqual([]);
  });

  it("should treat as 'left' when an invalid direction is passed", () => {
    // @ts-expect-error
    expect(drop([1, 2, 3], 1, "invalid")).toEqual([2, 3]);
  });
});
