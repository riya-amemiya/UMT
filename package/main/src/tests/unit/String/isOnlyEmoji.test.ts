import { isOnlyEmoji } from "@/String/isOnlyEmoji";

describe("isOnlyEmoji", () => {
  it("should return true for empty string", () => {
    expect(isOnlyEmoji("")).toBe(true);
  });

  it("should return true for string with only emojis", () => {
    expect(isOnlyEmoji("🎉")).toBe(true);
    expect(isOnlyEmoji("😀😃😄")).toBe(true);
    expect(isOnlyEmoji("🎈🎊🎉")).toBe(true);
    expect(isOnlyEmoji("❥･•❄")).toBe(true);
    expect(
      isOnlyEmoji(`🎉
        🎈
        🎊
        🎉
        `),
    ).toBe(true);
    expect(isOnlyEmoji("？？？？？")).toBe(true);
  });

  it("should return false for string with text", () => {
    expect(isOnlyEmoji("hello")).toBe(false);
    expect(isOnlyEmoji("test 123")).toBe(false);
    expect(isOnlyEmoji("こんにちわ")).toBe(false);
  });

  it("should return false for mixed content", () => {
    expect(isOnlyEmoji("hello 👋")).toBe(false);
    expect(isOnlyEmoji("🎉 party")).toBe(false);
    expect(isOnlyEmoji("123 🎈 abc")).toBe(false);
    expect(isOnlyEmoji("こんにちわ 😄")).toBe(false);
    expect(
      isOnlyEmoji(`
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
    expect(isOnlyEmoji("!@#$%")).toBe(true);
    expect(isOnlyEmoji("     ")).toBe(true);
  });
});
