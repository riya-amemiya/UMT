import { escapeRegExp } from "@/Tool/escapeRegExp";

describe("escapeRegExp", () => {
  it("should escape all regex special characters", () => {
    // biome-ignore lint/suspicious/noTemplateCurlyInString: Testing special characters
    const specialChars = ".*+?^${}()|[]\\";
    const expected = "\\.\\*\\+\\?\\^\\$\\{\\}\\(\\)\\|\\[\\]\\\\";
    expect(escapeRegExp(specialChars)).toBe(expected);
  });

  it("should escape each character correctly individually", () => {
    expect(escapeRegExp(".")).toBe("\\.");
    expect(escapeRegExp("*")).toBe("\\*");
    expect(escapeRegExp("+")).toBe("\\+");
    expect(escapeRegExp("?")).toBe("\\?");
    expect(escapeRegExp("^")).toBe("\\^");
    expect(escapeRegExp("$")).toBe("\\$");
    expect(escapeRegExp("{")).toBe("\\{");
    expect(escapeRegExp("}")).toBe("\\}");
    expect(escapeRegExp("(")).toBe("\\(");
    expect(escapeRegExp(")")).toBe("\\)");
    expect(escapeRegExp("|")).toBe("\\|");
    expect(escapeRegExp("[")).toBe("\\[");
    expect(escapeRegExp("]")).toBe("\\]");
    expect(escapeRegExp("\\")).toBe("\\\\");
  });

  it("should not escape alphanumeric characters", () => {
    const string_ = "abcABC123";
    expect(escapeRegExp(string_)).toBe(string_);
  });

  it("should handle mixed strings", () => {
    const string_ = "a.b+c";
    expect(escapeRegExp(string_)).toBe("a\\.b\\+c");
  });

  it("should handle empty string", () => {
    expect(escapeRegExp("")).toBe("");
  });
});
