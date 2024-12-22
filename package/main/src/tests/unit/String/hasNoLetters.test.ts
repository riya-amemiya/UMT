import { hasNoLetters } from "@/String/hasNoLetters";

describe("hasNoLetters", () => {
  it("should return true for empty string", () => {
    expect(hasNoLetters("")).toBe(true);
  });

  it("should return true for string with only emojis", () => {
    expect(hasNoLetters("🎉")).toBe(true);
    expect(hasNoLetters("😀😃😄")).toBe(true);
    expect(hasNoLetters("🎈🎊🎉")).toBe(true);
    expect(hasNoLetters("❥･•❄")).toBe(true);
    expect(
      hasNoLetters(`🎉
        🎈
        🎊
        🎉
        `),
    ).toBe(true);
    expect(hasNoLetters("？？？？？")).toBe(true);
    expect(hasNoLetters("12345")).toBe(true);
  });

  it("should return false for string with text", () => {
    expect(hasNoLetters("hello")).toBe(false);
    expect(hasNoLetters("test 123")).toBe(false);
    expect(hasNoLetters("こんにちわ")).toBe(false);
  });

  it("should return false for mixed content", () => {
    expect(hasNoLetters("hello 👋")).toBe(false);
    expect(hasNoLetters("🎉 party")).toBe(false);
    expect(hasNoLetters("123 🎈 abc")).toBe(false);
    expect(hasNoLetters("こんにちわ 😄")).toBe(false);
    expect(
      hasNoLetters(`
        こんにちわ
        私はHogehoge
        😄
        😄
        😄
        😄
        `),
    ).toBe(false);
  });

  it("should return false for special characters", () => {
    expect(hasNoLetters("!@#$%")).toBe(true);
    expect(hasNoLetters("     ")).toBe(true);
  });
});
