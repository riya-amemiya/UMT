import { mask } from "@/String/mask";

describe("mask", () => {
  it("masks the middle with default options", () => {
    expect(mask("secret")).toBe("s****t");
  });

  it("masks with custom start and end", () => {
    expect(mask("1234567890", { start: 2, end: 4 })).toBe("12****7890");
  });

  it("uses custom mask character", () => {
    expect(mask("secret", { char: "#" })).toBe("s####t");
  });

  it("returns the input unchanged when start+end >= length", () => {
    expect(mask("abc", { start: 2, end: 1 })).toBe("abc");
  });

  it("returns the input unchanged when start+end exceeds length", () => {
    expect(mask("abc", { start: 5, end: 5 })).toBe("abc");
  });

  it("handles surrogate pairs as single graphemes", () => {
    const input = "\u{1F600}abcde\u{1F600}";
    const expected = "\u{1F600}*****\u{1F600}";
    expect(mask(input, { start: 1, end: 1 })).toBe(expected);
  });

  it("returns the input unchanged for empty string", () => {
    expect(mask("")).toBe("");
  });
});
