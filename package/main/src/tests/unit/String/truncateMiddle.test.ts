import { truncateMiddle } from "@/String/truncateMiddle";

describe("truncateMiddle", () => {
  it("should truncate according to JSDoc examples", () => {
    expect(truncateMiddle("Hello World", 8)).toBe("Hel...ld");
    expect(truncateMiddle("Hello World", 7, "~")).toBe("Hel~rld");
    expect(truncateMiddle("Hello", 10)).toBe("Hello");
    expect(truncateMiddle("Hello World", 2)).toBe("..");
  });

  it("should not truncate when the string fits", () => {
    expect(truncateMiddle("Hello World", 11)).toBe("Hello World");
    expect(truncateMiddle("", 0)).toBe("");
  });

  it("should give the extra character to the head", () => {
    expect(truncateMiddle("Hello World", 10)).toBe("Hell...rld");
    expect(truncateMiddle("Hello World", 4)).toBe("H...");
  });

  it("should never exceed the requested length", () => {
    const input = "abcdefghijklmnopqrstuvwxyz";
    for (let length = 0; length <= input.length; length++) {
      expect(truncateMiddle(input, length).length).toBeLessThanOrEqual(length);
    }
  });

  it("should cut the separator when length is not larger than it", () => {
    expect(truncateMiddle("Hello World", 3)).toBe("...");
    expect(truncateMiddle("Hello World", 1)).toBe(".");
    expect(truncateMiddle("Hello World", 0)).toBe("");
    expect(truncateMiddle("Hello World", -1)).toBe("");
  });

  it("should handle an empty separator", () => {
    expect(truncateMiddle("abcdef", 4, "")).toBe("abef");
    expect(truncateMiddle("abcdef", 0, "")).toBe("");
  });

  it("should count UTF-16 code units", () => {
    expect(truncateMiddle("a\u{1f600}b", 3, "")).toBe("a\uD83Db");
  });
});
