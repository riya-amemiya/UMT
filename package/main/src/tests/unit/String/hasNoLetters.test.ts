import { hasNoLetters } from "@/String/hasNoLetters";

describe("hasNoLetters", () => {
  it("should return true for strings without any letters", () => {
    expect(hasNoLetters("")).toBe(true); // Empty string
    expect(hasNoLetters("12345")).toBe(true); // Numbers only
    expect(hasNoLetters("!@#$%")).toBe(true); // Special characters
    expect(hasNoLetters("     ")).toBe(true); // Whitespace only
  });

  it("should return true for strings with only emojis and symbols", () => {
    expect(hasNoLetters("🎉")).toBe(true); // Single emoji
    expect(hasNoLetters("😀😃😄")).toBe(true); // Multiple emojis
    expect(hasNoLetters("❥･•❄")).toBe(true); // Decorative symbols
    expect(hasNoLetters("？？？？？")).toBe(true); // Full-width symbols
    expect(
      hasNoLetters(`
      🎈
      🎊
      🎉
    `),
    ).toBe(true); // Multiline emojis
  });

  it("should return false for strings containing letters", () => {
    expect(hasNoLetters("hello")).toBe(false); // English text
    expect(hasNoLetters("test 123")).toBe(false); // Mixed with numbers
    expect(hasNoLetters("こんにちは")).toBe(false); // Japanese text
    expect(hasNoLetters("Café")).toBe(false); // Accented characters
  });

  it("should return false for strings with mixed content", () => {
    expect(hasNoLetters("hello 👋")).toBe(false); // Text with emoji
    expect(hasNoLetters("🎉 party")).toBe(false); // Emoji with text
    expect(
      hasNoLetters(`
      Hello
      World
      😄
      123
    `),
    ).toBe(false); // Multiline mixed content
  });
});
