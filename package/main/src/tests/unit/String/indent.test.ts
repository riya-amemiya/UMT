import { dedent } from "@/String/dedent";
import { indent } from "@/String/indent";

describe("indent", () => {
  it("should indent according to JSDoc examples", () => {
    expect(indent("line1\nline2", 2)).toBe("  line1\n  line2");
    expect(indent("a\n\nb", 1, "\t")).toBe("\ta\n\n\tb");
  });

  it("should prefix whitespace-only lines because they are not empty", () => {
    expect(indent("  \nx", 2)).toBe("    \n  x");
  });

  it("should repeat a multi-character unit", () => {
    expect(indent("x", 2, "ab")).toBe("ababx");
  });

  it("should leave the string unchanged for zero or negative count", () => {
    expect(indent("a\nb", 0)).toBe("a\nb");
    expect(indent("a\nb", -1)).toBe("a\nb");
  });

  it("should leave an empty string unchanged", () => {
    expect(indent("", 4)).toBe("");
    expect(indent("\n", 4)).toBe("\n");
  });

  it("should be reversed by dedent", () => {
    const original = "a\n  b\n\nc";
    expect(dedent(indent(original, 4))).toBe(original);
  });
});
