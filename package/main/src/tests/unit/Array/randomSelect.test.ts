import { randomSelect } from "@/Array/randomSelect";

describe("randomSelect", () => {
  it("should randomly select the specified number of elements", () => {
    const array = [1, 2, 3, 4, 5];
    const result = randomSelect(array, 2);
    expect(result).toHaveLength(2);
    result.forEach((item) => {
      expect(array).toContain(item);
    });
  });

  it("should return elements equal to array length when count exceeds array length", () => {
    const array = [1, 2, 3];
    const result = randomSelect(array, 5);
    expect(result).toHaveLength(3);
    result.forEach((item) => {
      expect(array).toContain(item);
    });
  });

  it("should return an empty array when given an empty array", () => {
    const result = randomSelect([], 3);
    expect(result).toEqual([]);
  });

  it("should return specified count with duplicates when allowDuplicates is true", () => {
    const array = [1, 2, 3];
    const result = randomSelect(array, 5, true);
    expect(result).toHaveLength(5);
    result.forEach((item) => {
      expect(array).toContain(item);
    });
  });

  it("should return specified count without duplicates when allowDuplicates is false", () => {
    const array = [1, 2, 3, 4, 5];
    const result = randomSelect(array, 3, false);
    expect(result).toHaveLength(3);
    result.forEach((item) => {
      expect(array).toContain(item);
    });
  });

  it("should return an empty array when count is negative", () => {
    const array = [1, 2, 3, 4, 5];
    const result = randomSelect(array, -1);
    expect(result).toEqual([]);
  });

  it("should return an empty array when count is zero", () => {
    const array = [1, 2, 3, 4, 5];
    const result = randomSelect(array, 0);
    expect(result).toEqual([]);
  });
});
