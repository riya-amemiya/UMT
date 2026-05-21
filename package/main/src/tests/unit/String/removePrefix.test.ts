import { removePrefix } from "@/String/removePrefix";

describe("removePrefix", () => {
  it("should remove the prefix when present", () => {
    expect(removePrefix("https://example.com", "https://")).toBe("example.com");
  });

  it("should leave the string unchanged when the prefix is absent", () => {
    expect(removePrefix("example.com", "https://")).toBe("example.com");
  });

  it("should remove the prefix only once", () => {
    expect(removePrefix("//path", "/")).toBe("/path");
  });

  it("should return the string unchanged for an empty prefix", () => {
    expect(removePrefix("abc", "")).toBe("abc");
  });

  it("should handle empty input string", () => {
    expect(removePrefix("", "x")).toBe("");
  });
});
