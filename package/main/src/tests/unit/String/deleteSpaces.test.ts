import { deleteSpaces } from "@/String/deleteSpaces";

describe("deleteSpaces", () => {
  it("should remove spaces from a string", () => {
    expect(deleteSpaces("Hello World")).toBe("HelloWorld");
    expect(deleteSpaces("   Hello   World   ")).toBe("HelloWorld");
  });

  it("should remove tabs and other whitespace characters", () => {
    expect(deleteSpaces("Hello\tWorld")).toBe("HelloWorld");
    expect(deleteSpaces("Hello\nWorld\r\n")).toBe("HelloWorld");
    expect(deleteSpaces("Hello\u2003World")).toBe("HelloWorld"); // em space
  });

  it("should handle empty string", () => {
    expect(deleteSpaces("")).toBe("");
    expect(deleteSpaces("   ")).toBe("");
  });

  it("should handle string with multibyte characters", () => {
    expect(deleteSpaces("こんにちは 世界")).toBe("こんにちは世界");
    expect(deleteSpaces("Hello　World")).toBe("HelloWorld"); // Full-width space
  });
});
