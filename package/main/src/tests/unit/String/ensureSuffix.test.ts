import { ensureSuffix } from "@/String/ensureSuffix";

describe("ensureSuffix", () => {
  it("should append the suffix when missing", () => {
    expect(ensureSuffix("file", ".txt")).toBe("file.txt");
  });

  it("should not duplicate an existing suffix", () => {
    expect(ensureSuffix("file.txt", ".txt")).toBe("file.txt");
  });

  it("should handle empty input string", () => {
    expect(ensureSuffix("", "/")).toBe("/");
  });

  it("should return the string unchanged for an empty suffix", () => {
    expect(ensureSuffix("abc", "")).toBe("abc");
  });
});
