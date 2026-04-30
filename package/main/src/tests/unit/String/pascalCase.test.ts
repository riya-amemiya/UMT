import { pascalCase } from "@/String/pascalCase";

describe("pascalCase", () => {
  it("converts kebab-case", () => {
    expect(pascalCase("hello-world")).toBe("HelloWorld");
  });

  it("converts snake_case", () => {
    expect(pascalCase("hello_world")).toBe("HelloWorld");
  });

  it("converts space-separated words", () => {
    expect(pascalCase("hello world")).toBe("HelloWorld");
  });

  it("converts camelCase to PascalCase", () => {
    expect(pascalCase("helloWorld")).toBe("HelloWorld");
  });

  it("returns empty for empty string", () => {
    expect(pascalCase("")).toBe("");
  });

  it("handles acronyms", () => {
    expect(pascalCase("XMLHttpRequest")).toBe("XmlHttpRequest");
  });
});
