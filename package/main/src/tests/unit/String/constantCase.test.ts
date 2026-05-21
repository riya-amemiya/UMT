import { constantCase } from "@/String/constantCase";

describe("constantCase", () => {
  it("should convert camelCase to CONSTANT_CASE", () => {
    expect(constantCase("helloWorld")).toBe("HELLO_WORLD");
  });

  it("should convert space-separated words", () => {
    expect(constantCase("Hello World")).toBe("HELLO_WORLD");
  });

  it("should convert kebab-case", () => {
    expect(constantCase("foo-bar-baz")).toBe("FOO_BAR_BAZ");
  });

  it("should handle a single word", () => {
    expect(constantCase("hello")).toBe("HELLO");
  });

  it("should handle empty string", () => {
    expect(constantCase("")).toBe("");
  });
});
