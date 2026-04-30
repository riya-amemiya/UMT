import { capitalizeWord } from "@/String/capitalizeWord";

describe("capitalizeWord", () => {
  it("uppercases the first letter and lowercases the rest", () => {
    expect(capitalizeWord("hello")).toBe("Hello");
  });

  it("normalizes mixed case input", () => {
    expect(capitalizeWord("hELLO")).toBe("Hello");
  });

  it("returns empty for empty input", () => {
    expect(capitalizeWord("")).toBe("");
  });

  it("preserves single-character input", () => {
    expect(capitalizeWord("a")).toBe("A");
  });
});
