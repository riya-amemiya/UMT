import { kebabCase } from "@/String/kebabCase";

describe("kebabCase", () => {
  it("should convert camelCase to kebab-case", () => {
    expect(kebabCase("helloWorld")).toBe("hello-world");
    expect(kebabCase("fooBarBaz")).toBe("foo-bar-baz");
  });

  it("should convert PascalCase to kebab-case", () => {
    expect(kebabCase("HelloWorld")).toBe("hello-world");
    expect(kebabCase("FooBarBaz")).toBe("foo-bar-baz");
  });

  it("should convert snake_case to kebab-case", () => {
    expect(kebabCase("hello_world")).toBe("hello-world");
    expect(kebabCase("foo_bar_baz")).toBe("foo-bar-baz");
  });

  it("should convert space-separated words to kebab-case", () => {
    expect(kebabCase("hello world")).toBe("hello-world");
    expect(kebabCase("foo bar baz")).toBe("foo-bar-baz");
  });

  it("should handle already kebab-case input", () => {
    expect(kebabCase("hello-world")).toBe("hello-world");
    expect(kebabCase("foo-bar-baz")).toBe("foo-bar-baz");
  });

  it("should handle mixed separators", () => {
    expect(kebabCase("helloWorld_test case")).toBe("hello-world-test-case");
    expect(kebabCase("fooBar-baz qux")).toBe("foo-bar-baz-qux");
  });

  it("should handle numbers", () => {
    expect(kebabCase("helloWorld2")).toBe("hello-world2");
    expect(kebabCase("testCase123")).toBe("test-case123");
  });

  it("should handle empty string", () => {
    expect(kebabCase("")).toBe("");
  });

  it("should handle single word", () => {
    expect(kebabCase("hello")).toBe("hello");
    expect(kebabCase("HELLO")).toBe("hello");
  });

  it("should remove special characters", () => {
    expect(kebabCase("hello@world")).toBe("hello-world");
    expect(kebabCase("foo#bar$baz")).toBe("foo-bar-baz");
  });

  it("should handle leading/trailing separators", () => {
    expect(kebabCase("-hello-world-")).toBe("hello-world");
    expect(kebabCase("_foo_bar_")).toBe("foo-bar");
  });

  it("should handle multiple consecutive separators", () => {
    expect(kebabCase("hello---world")).toBe("hello-world");
    expect(kebabCase("foo___bar")).toBe("foo-bar");
  });

  it("should handle complex mixed case", () => {
    expect(kebabCase("XMLHttpRequest")).toBe("xml-http-request");
    expect(kebabCase("getElementById")).toBe("get-element-by-id");
  });

  it("should handle acronyms", () => {
    expect(kebabCase("HTML")).toBe("html");
    expect(kebabCase("XMLParser")).toBe("xml-parser");
  });
});
