import { dedent } from "@/String/dedent";

describe("dedent", () => {
  it("removes minimum common indentation", () => {
    const result = dedent("    line1\n      line2\n    line3");
    expect(result).toBe("line1\n  line2\nline3");
  });

  it("ignores blank lines when computing minimum", () => {
    const result = dedent("    a\n\n    b");
    expect(result).toBe("a\n\nb");
  });

  it("returns the input unchanged when no indentation exists", () => {
    expect(dedent("no indent")).toBe("no indent");
  });

  it("handles all-blank input", () => {
    expect(dedent("\n  \n")).toBe("\n  \n");
  });

  it("works as a tagged template", () => {
    const value = 42;
    const result = dedent`
      number: ${value}
    `;
    expect(result).toBe("\nnumber: 42\n");
  });

  it("interpolates multiple values", () => {
    const result = dedent`
      ${"a"} and ${"b"}
    `;
    expect(result).toBe("\na and b\n");
  });
});
