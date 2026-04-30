import { uncapitalize } from "@/String/uncapitalize";

describe("uncapitalize", () => {
  it("lowercases the first letter", () => {
    expect(uncapitalize("Hello")).toBe("hello");
  });

  it("preserves the rest of the string", () => {
    expect(uncapitalize("ÉCLAIR")).toBe("éCLAIR");
  });

  it("returns empty for empty input", () => {
    expect(uncapitalize("")).toBe("");
  });

  it("handles surrogate pair first character", () => {
    const input = "\u{1F600}ABC";
    expect(uncapitalize(input)).toBe(input);
  });
});
