import { slugify } from "@/String/slugify";

describe("slugify", () => {
  it("should convert text to URL-friendly slug according to JSDoc examples", () => {
    expect(slugify("Hello World!")).toBe("hello-world");
    expect(slugify("This is a Test")).toBe("this-is-a-test");
    expect(slugify("Japanese: こんにちは")).toBe("japanese");
  });

  it("should handle multiple spaces", () => {
    expect(slugify("Hello    World")).toBe("hello-world");
  });

  it("should handle leading and trailing spaces", () => {
    expect(slugify("  Hello World  ")).toBe("hello-world");
  });

  it("should remove special characters", () => {
    expect(slugify("Special!@#$%Characters")).toBe("special-characters");
  });

  it("should handle underscores", () => {
    expect(slugify("snake_case_text")).toBe("snake-case-text");
  });

  it("should handle unicode characters", () => {
    expect(slugify("café")).toBe("cafe");
    expect(slugify("naïve")).toBe("naive");
  });

  it("should handle numbers", () => {
    expect(slugify("Test 123")).toBe("test-123");
    expect(slugify("Version 2.5")).toBe("version-2-5");
  });

  it("should handle empty string", () => {
    expect(slugify("")).toBe("");
  });

  it("should handle consecutive hyphens", () => {
    expect(slugify("Hello---World")).toBe("hello-world");
  });

  it("should handle mixed case", () => {
    expect(slugify("CamelCase")).toBe("camelcase");
  });
});
