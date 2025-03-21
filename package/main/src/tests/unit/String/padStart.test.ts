import { padStart } from "@/String/padStart";

describe("padStart", () => {
  describe("basic functionality", () => {
    it("should pad the start of string with specified character", () => {
      expect(padStart("abc", 5, " ")).toBe("  abc");
      expect(padStart("abc", 5, "0")).toBe("00abc");
    });

    it("should return original string if already at or exceeding target length", () => {
      expect(padStart("abc", 2, " ")).toBe("abc"); // Shorter target
      expect(padStart("abc", 3, " ")).toBe("abc"); // Equal length
      expect(padStart("abc", 0, " ")).toBe("abc"); // Zero target
    });

    it("should handle empty string input", () => {
      expect(padStart("", 3, "0")).toBe("000");
      expect(padStart("", 4, "x")).toBe("xxxx");
    });
  });

  describe("multi-character padding", () => {
    it("should repeat padding string to reach target length", () => {
      expect(padStart("abc", 7, "xy")).toBe("xyxyabc");
      expect(padStart("abc", 8, "123")).toBe("12312abc");
    });

    it("should truncate padding string if needed", () => {
      expect(padStart("abc", 6, "12345")).toBe("123abc");
      expect(padStart("abc", 5, "0000")).toBe("00abc");
    });

    it("should work with single-character padding string", () => {
      expect(padStart("abc", 5, "x")).toBe("xxabc");
    });
  });

  describe("error handling", () => {
    it("should throw error on empty padding string", () => {
      expect(() => padStart("abc", 5, "")).toThrow("padString cannot be empty");
    });
  });
});
