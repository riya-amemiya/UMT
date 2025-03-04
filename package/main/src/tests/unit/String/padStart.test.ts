import { padStart } from "@/String/padStart";

describe("padStart function", () => {
  describe("basic padding functionality", () => {
    it("should pad the string with the given padString to the targetLength", () => {
      expect(padStart("abc", 5, " ")).toBe("  abc");
      expect(padStart("abc", 5, "0")).toBe("00abc");
    });

    it("should return original string when targetLength is less than string length", () => {
      expect(padStart("abc", 2, " ")).toBe("abc");
    });

    it("should return original string when targetLength equals string length", () => {
      expect(padStart("abc", 3, " ")).toBe("abc");
    });
  });

  describe("padding with longer strings", () => {
    it("should repeat the padString if necessary", () => {
      expect(padStart("abc", 7, "xy")).toBe("xyxyabc");
      expect(padStart("abc", 8, "123")).toBe("12312abc");
    });

    it("should truncate the padString if the combined length exceeds the targetLength", () => {
      expect(padStart("abc", 6, "12345")).toBe("123abc");
      expect(padStart("abc", 5, "0000")).toBe("00abc");
    });
  });

  describe("edge cases", () => {
    it("should handle targetLength of 0", () => {
      expect(padStart("abc", 0, " ")).toBe("abc");
    });

    it("should handle empty string input", () => {
      expect(padStart("", 3, "0")).toBe("000");
    });

    it("should throw an error if the padString is empty", () => {
      expect(() => padStart("abc", 5, "")).toThrow("padString cannot be empty");
    });

    it("should handle single character padding", () => {
      expect(padStart("abc", 5, "x")).toBe("xxabc");
    });
  });
});
