import { removeSuffix } from "@/String/removeSuffix";

describe("removeSuffix", () => {
  it("should remove the suffix when present", () => {
    expect(removeSuffix("file.txt", ".txt")).toBe("file");
  });

  it("should leave the string unchanged when the suffix is absent", () => {
    expect(removeSuffix("file", ".txt")).toBe("file");
  });

  it("should remove the suffix only once", () => {
    expect(removeSuffix("path//", "/")).toBe("path/");
  });

  it("should return the string unchanged for an empty suffix", () => {
    expect(removeSuffix("abc", "")).toBe("abc");
  });

  it("should handle empty input string", () => {
    expect(removeSuffix("", "x")).toBe("");
  });
});
