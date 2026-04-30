import { capitalize } from "@/String/capitalize";

describe("capitalize", () => {
  it("uppercases the first letter", () => {
    expect(capitalize("hello")).toBe("Hello");
  });

  it("preserves the rest of the string", () => {
    expect(capitalize("hELLO")).toBe("HELLO");
  });

  it("handles accented first letter", () => {
    expect(capitalize("éclair")).toBe("Éclair");
  });

  it("returns empty for empty input", () => {
    expect(capitalize("")).toBe("");
  });

  it("handles surrogate pair first character", () => {
    const input = "\u{1F600}abc";
    expect(capitalize(input)).toBe(input);
  });
});
