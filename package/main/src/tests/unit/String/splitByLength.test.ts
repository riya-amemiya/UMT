import { splitByLength } from "@/String/splitByLength";

describe("splitByLength", () => {
  it("should split into equal chunks", () => {
    expect(splitByLength("abcdef", 2)).toEqual(["ab", "cd", "ef"]);
  });

  it("should keep the remainder in the last chunk", () => {
    expect(splitByLength("abcdefg", 3)).toEqual(["abc", "def", "g"]);
  });

  it("should return a single chunk when length exceeds the string", () => {
    expect(splitByLength("abc", 10)).toEqual(["abc"]);
  });

  it("should be surrogate-pair safe", () => {
    expect(splitByLength("😀😁😂😃", 2)).toEqual(["😀😁", "😂😃"]);
  });

  it("should return an empty array for an empty string", () => {
    expect(splitByLength("", 3)).toEqual([]);
  });
});
