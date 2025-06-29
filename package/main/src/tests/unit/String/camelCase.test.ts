import { camelCase } from "@/String/camelCase";

describe("camelCase", () => {
  it("should convert kebab-case to camelCase", () => {
    expect(camelCase("hello-world")).toBe("helloWorld");
    expect(camelCase("foo-bar-baz")).toBe("fooBarBaz");
  });

  it("should convert snake_case to camelCase", () => {
    expect(camelCase("hello_world")).toBe("helloWorld");
    expect(camelCase("foo_bar_baz")).toBe("fooBarBaz");
  });

  it("should convert space-separated words to camelCase", () => {
    expect(camelCase("hello world")).toBe("helloWorld");
    expect(camelCase("foo bar baz")).toBe("fooBarBaz");
  });

  it("should handle PascalCase input", () => {
    expect(camelCase("HelloWorld")).toBe("helloWorld");
    expect(camelCase("FooBarBaz")).toBe("fooBarBaz");
  });

  it("should handle already camelCase input", () => {
    expect(camelCase("helloWorld")).toBe("helloWorld");
    expect(camelCase("fooBarBaz")).toBe("fooBarBaz");
  });

  it("should handle mixed separators", () => {
    expect(camelCase("hello-world_test case")).toBe("helloWorldTestCase");
    expect(camelCase("foo_bar-baz qux")).toBe("fooBarBazQux");
  });

  it("should handle numbers", () => {
    expect(camelCase("hello-world-2")).toBe("helloWorld2");
    expect(camelCase("test_case_123")).toBe("testCase123");
  });

  it("should handle empty string", () => {
    expect(camelCase("")).toBe("");
  });

  it("should handle single word", () => {
    expect(camelCase("hello")).toBe("hello");
    expect(camelCase("HELLO")).toBe("hELLO");
  });

  it("should handle special characters", () => {
    expect(camelCase("hello@world")).toBe("helloWorld");
    expect(camelCase("foo#bar$baz")).toBe("fooBarBaz");
  });

  it("should handle leading/trailing separators", () => {
    expect(camelCase("-hello-world-")).toBe("helloWorld");
    expect(camelCase("_foo_bar_")).toBe("fooBar");
  });

  it("should handle multiple consecutive separators", () => {
    expect(camelCase("hello---world")).toBe("helloWorld");
    expect(camelCase("foo___bar")).toBe("fooBar");
  });
});
