import { buildUrl } from "@/URL/buildUrl";

describe("buildUrl", () => {
  it("should build a URL with query parameters", () => {
    const result = buildUrl("https://example.com", {
      page: "1",
      q: "search",
    });
    expect(result).toBe("https://example.com/?page=1&q=search");
  });

  it("should return the base URL when no params given", () => {
    const result = buildUrl("https://example.com/path");
    expect(result).toBe("https://example.com/path");
  });

  it("should return the base URL with empty params", () => {
    const result = buildUrl("https://example.com/path", {});
    expect(result).toBe("https://example.com/path");
  });

  it("should encode special characters in values", () => {
    const result = buildUrl("https://example.com", {
      q: "hello world",
    });
    expect(result).toContain("q=hello+world");
  });

  it("should handle existing query parameters in base", () => {
    const result = buildUrl("https://example.com?existing=1", {
      added: "2",
    });
    expect(result).toContain("existing=1");
    expect(result).toContain("added=2");
  });

  it("should handle a single parameter", () => {
    const result = buildUrl("https://example.com", { key: "value" });
    expect(result).toBe("https://example.com/?key=value");
  });
});
