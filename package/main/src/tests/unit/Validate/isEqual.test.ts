import { isEqual } from "@/Validate/isEqual";
import _ from "lodash";
describe("isEqual", () => {
  it("should compare primitive values correctly", () => {
    expect(isEqual(1, 1)).toBe(true);
    expect(isEqual("test", "test")).toBe(true);
    expect(isEqual(true, true)).toBe(true);
    expect(isEqual(null, null)).toBe(true);
    expect(isEqual(undefined, undefined)).toBe(true);
    expect(isEqual(Number.NaN, Number.NaN)).toBe(true);
  });

  it("should return false for different primitive values", () => {
    expect(isEqual(1, 2)).toBe(false);
    expect(isEqual("test", "other")).toBe(false);
    expect(isEqual(true, false)).toBe(false);
    expect(isEqual(null, undefined)).toBe(false);
  });

  it("should handle object references correctly", () => {
    const obj = { a: 1 };
    expect(isEqual(obj, obj)).toBe(true);
    expect(isEqual(obj, { a: 1 })).toBe(false);
  });

  it("should handle array references correctly", () => {
    const arr = [1, 2, 3];
    expect(isEqual(arr, arr)).toBe(true);
    expect(isEqual(arr, [1, 2, 3])).toBe(false);
  });

  it("should distinguish between -0 and +0", () => {
    expect(isEqual(-0, +0)).toBe(false);
  });

  // lodash tests
  it("should compare primitive values correctly (using lodash)", () => {
    expect(_.isEqual(1, 1)).toBe(true);
    expect(_.isEqual("test", "test")).toBe(true);
    expect(_.isEqual(true, true)).toBe(true);
    expect(_.isEqual(null, null)).toBe(true);
    expect(_.isEqual(undefined, undefined)).toBe(true);
    expect(_.isEqual(Number.NaN, Number.NaN)).toBe(true);
  });

  it("should return false for different primitive values (using lodash)", () => {
    expect(_.isEqual(1, 2)).toBe(false);
    expect(_.isEqual("test", "other")).toBe(false);
    expect(_.isEqual(true, false)).toBe(false);
    expect(_.isEqual(null, undefined)).toBe(false);
  });

  it("should handle object references correctly (using lodash)", () => {
    const obj = { a: 1 };
    expect(_.isEqual(obj, obj)).toBe(true);
    // This test returns true because the implementation of lodash and isEqual is different.
    expect(_.isEqual(obj, { a: 1 })).toBe(true);
  });

  it("should handle array references correctly (using lodash)", () => {
    const arr = [1, 2, 3];
    expect(_.isEqual(arr, arr)).toBe(true);
    // This test returns true because the implementation of lodash and isEqual is different.
    expect(_.isEqual(arr, [1, 2, 3])).toBe(true);
  });

  it("should distinguish between -0 and +0 (using lodash)", () => {
    // This test returns true because the implementation of lodash and isEqual is different.
    expect(_.isEqual(-0, +0)).toBe(true);
  });
});
