import { toBaseN } from "@/Math/toBaseN";

describe("toBaseN function", () => {
  describe("binary (base 2)", () => {
    it("should convert decimal numbers to binary strings", () => {
      expect(toBaseN(1)).toBe("1");
      expect(toBaseN(2)).toBe("10");
      expect(toBaseN(3)).toBe("11");
      expect(toBaseN(4)).toBe("100");
      expect(toBaseN(5)).toBe("101");
      expect(toBaseN(6)).toBe("110");
      expect(toBaseN(112)).toBe("1110000");
    });
  });

  describe("base 4", () => {
    it("should convert decimal numbers to base 4 strings", () => {
      expect(toBaseN(7, 4)).toBe("13");
      expect(toBaseN(8, 4)).toBe("20");
      expect(toBaseN(9, 4)).toBe("21");
      expect(toBaseN(112, 4)).toBe("1300");
    });
  });

  describe("octal (base 8)", () => {
    it("should convert decimal numbers to octal strings", () => {
      expect(toBaseN(7, 8)).toBe("7");
      expect(toBaseN(8, 8)).toBe("10");
      expect(toBaseN(9, 8)).toBe("11");
      expect(toBaseN(112, 8)).toBe("160");
    });
  });

  describe("hexadecimal (base 16)", () => {
    it("should convert decimal numbers to hexadecimal strings", () => {
      expect(toBaseN(10, 16)).toBe("a");
      expect(toBaseN(15, 16)).toBe("f");
      expect(toBaseN(16, 16)).toBe("10");
      expect(toBaseN(255, 16)).toBe("ff");
    });
  });
});
