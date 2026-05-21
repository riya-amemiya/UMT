import { ensurePrefix } from "@/String/ensurePrefix";

describe("ensurePrefix", () => {
  it("should prepend the prefix when missing", () => {
    expect(ensurePrefix("example.com", "https://")).toBe("https://example.com");
  });

  it("should not duplicate an existing prefix", () => {
    expect(ensurePrefix("https://example.com", "https://")).toBe(
      "https://example.com",
    );
  });

  it("should handle empty input string", () => {
    expect(ensurePrefix("", "/")).toBe("/");
  });

  it("should return the string unchanged for an empty prefix", () => {
    expect(ensurePrefix("abc", "")).toBe("abc");
  });
});
