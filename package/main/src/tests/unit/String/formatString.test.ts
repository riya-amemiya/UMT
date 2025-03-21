import { formatString } from "@/String/formatString";

describe("formatString", () => {
  it("should replace placeholders with specified values", () => {
    const template = "Hello, {0}! It's {1} today.";
    const result = formatString(template, "World", "sunny");
    expect(result).toBe("Hello, World! It's sunny today.");
  });

  it("should not replace placeholders with undefined values", () => {
    const template = "Hello, {0}! How's {1}?";
    const result = formatString(template, "World");
    expect(result).toBe("Hello, World! How's {1}?");
  });

  it("should not replace placeholders with non-existent indices", () => {
    const template = "Hello, {0}! {2} is not available.";
    const result = formatString(template, "World", "sunny");
    expect(result).toBe("Hello, World! {2} is not available.");
  });

  it("should handle special characters in placeholder values", () => {
    const template = "Hello {0}! Special chars: {1}";
    const result = formatString(template, "🌍", "©®™%$#@!");
    expect(result).toBe("Hello 🌍! Special chars: ©®™%$#@!");
  });

  it("should handle empty strings as values", () => {
    const template = "Start{0}Middle{1}End";
    const result = formatString(template, "", "");
    expect(result).toBe("StartMiddleEnd");
  });

  it("should convert non-string values to strings", () => {
    const template = "Number: {0}, Object: {1}, Array: {2}";
    const result = formatString(template, 123, { key: "value" }, [1, 2, 3]);
    expect(result).toBe("Number: 123, Object: [object Object], Array: 1,2,3");
  });

  it("should handle repeated placeholders", () => {
    const template = "{0} {1} {0} {1} {0}";
    const result = formatString(template, "A", "B");
    expect(result).toBe("A B A B A");
  });

  it("should handle whitespace characters in values", () => {
    const template = "Space:{0}, Tab:{1}, Newline:{2}";
    const result = formatString(template, " ", "\t", "\n");
    expect(result).toBe("Space: , Tab:\t, Newline:\n");
  });
});
