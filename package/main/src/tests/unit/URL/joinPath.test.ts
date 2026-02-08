import { joinPath } from "@/URL/joinPath";

describe("joinPath", () => {
  it("should join simple segments", () => {
    expect(joinPath("a", "b", "c")).toBe("a/b/c");
  });

  it("should normalize slashes between segments", () => {
    expect(joinPath("a/", "/b/", "/c")).toBe("a/b/c");
  });

  it("should preserve leading slash of first segment", () => {
    expect(joinPath("/a", "b", "c")).toBe("/a/b/c");
  });

  it("should preserve trailing slash of last segment", () => {
    expect(joinPath("a", "b", "c/")).toBe("a/b/c/");
  });

  it("should handle URL-like base segments", () => {
    expect(joinPath("https://example.com/", "/api/", "/users")).toBe(
      "https://example.com/api/users",
    );
  });

  it("should handle single segment", () => {
    expect(joinPath("path")).toBe("path");
  });

  it("should return empty string for no segments", () => {
    expect(joinPath()).toBe("");
  });

  it("should handle multiple slashes", () => {
    expect(joinPath("a///", "///b")).toBe("a/b");
  });

  it("should handle empty segments", () => {
    expect(joinPath("a", "", "b")).toBe("a/b");
  });
});
