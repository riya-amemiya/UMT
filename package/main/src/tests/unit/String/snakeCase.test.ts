import { snakeCase } from "@/String/snakeCase";

describe("snakeCase", () => {
  it("converts camelCase", () => {
    expect(snakeCase("helloWorld")).toBe("hello_world");
  });

  it("converts space-separated words", () => {
    expect(snakeCase("Hello World")).toBe("hello_world");
  });

  it("converts kebab-case", () => {
    expect(snakeCase("foo-bar-baz")).toBe("foo_bar_baz");
  });

  it("converts PascalCase", () => {
    expect(snakeCase("FooBar")).toBe("foo_bar");
  });

  it("returns empty for empty string", () => {
    expect(snakeCase("")).toBe("");
  });

  it("handles acronyms", () => {
    expect(snakeCase("XMLHttpRequest")).toBe("xml_http_request");
  });
});
