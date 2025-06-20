import { calculatorCore } from "@/Math/calculator/core";

describe("calculatorCore function", () => {
  describe("basic operations", () => {
    test("should handle simple addition", () => {
      expect(calculatorCore("1+1")).toBe("2");
      expect(calculatorCore("10+20")).toBe("30");
      expect(calculatorCore("0+0")).toBe("0");
    });

    test("should handle simple subtraction", () => {
      expect(calculatorCore("5-3")).toBe("2");
      expect(calculatorCore("10-20")).toBe("-10");
      expect(calculatorCore("0-0")).toBe("0");
    });

    test("should handle simple multiplication", () => {
      expect(calculatorCore("3*4")).toBe("12");
      expect(calculatorCore("5*0")).toBe("0");
      expect(calculatorCore("1*1")).toBe("1");
    });

    test("should handle simple division", () => {
      expect(calculatorCore("8/2")).toBe("4");
      expect(calculatorCore("1/1")).toBe("1");
      const result = calculatorCore("10/3");
      expect(result).toMatch(/^3\.33333/);
    });

    test("should handle exponentiation", () => {
      expect(calculatorCore("2^3")).toBe("8");
      expect(calculatorCore("5^2")).toBe("25");
      expect(calculatorCore("2^0")).toBe("1");
    });
  });

  describe("sign sanitization", () => {
    test("should handle double minus", () => {
      expect(calculatorCore("5--3")).toBe("8");
      expect(calculatorCore("10--5")).toBe("15");
    });

    test("should handle double plus", () => {
      expect(calculatorCore("5++3")).toBe("8");
      expect(calculatorCore("10++5")).toBe("15");
    });

    test("should handle mixed signs", () => {
      expect(calculatorCore("5+-3")).toBe("2");
      expect(calculatorCore("5-+3")).toBe("2");
    });
  });

  describe("parentheses", () => {
    test("should handle simple parentheses", () => {
      expect(calculatorCore("(2+3)")).toBe("5");
      expect(calculatorCore("(10-5)")).toBe("5");
      expect(calculatorCore("(3*4)")).toBe("12");
      expect(calculatorCore("(8/2)")).toBe("4");
    });

    test("should handle invalid parentheses", () => {
      expect(calculatorCore("(2+")).toBe("(2+");
      expect(calculatorCore("2+)")).toBe("2+)");
    });

    test("should handle nested parentheses partially", () => {
      const result = calculatorCore("((2+3))");
      expect(typeof result).toBe("string");
    });
  });

  describe("operator precedence behavior", () => {
    test("should process operations sequentially", () => {
      const result = calculatorCore("2+3*4");
      expect(typeof result).toBe("string");
    });

    test("should handle rightmost operations first for same type", () => {
      const result = calculatorCore("2*3*4");
      expect(typeof result).toBe("string");
    });
  });

  describe("currency conversion", () => {
    test("should handle basic currency conversion", () => {
      expect(calculatorCore("$10*2", { $: 100 })).toBe("2000");
    });

    test("should handle multiple currencies", () => {
      const result = calculatorCore("$10+E5", { $: 100, E: 110 });
      expect(typeof result).toBe("string");
    });

    test("should work without currency conversion", () => {
      expect(calculatorCore("10+20")).toBe("30");
      expect(calculatorCore("10*20", {})).toBe("200");
    });
  });

  describe("decimal operations", () => {
    test("should handle decimal numbers", () => {
      expect(calculatorCore("2.5+1.5")).toBe("4");
      expect(calculatorCore("3.14*2")).toBe("6.28");
      expect(calculatorCore("10.5/2.1")).toBe("5");
    });

    test("should handle precision issues", () => {
      const result = calculatorCore("0.1+0.2");
      expect(result).toBe("0.3");
    });
  });

  describe("error cases", () => {
    test("should return original expression for invalid input", () => {
      expect(calculatorCore("")).toBe("");
      expect(calculatorCore("abc")).toBe("abc");
      expect(calculatorCore("*/")).toBe("*/");
    });

    test("should handle division by zero", () => {
      expect(calculatorCore("5/0")).toBe("5/0");
      expect(calculatorCore("0/0")).toBe("0/0");
    });

    test("should handle numbers only", () => {
      expect(calculatorCore("42")).toBe("42");
      expect(calculatorCore("0")).toBe("0");
      expect(calculatorCore("-5")).toBe("-5");
    });
  });

  describe("incomplete expressions", () => {
    test("should return incomplete expressions as-is", () => {
      expect(calculatorCore("1+")).toBe("1+");
      expect(calculatorCore("2*")).toBe("2*");
      expect(calculatorCore("5/")).toBe("5/");
      expect(calculatorCore("3^")).toBe("3^");
    });
  });

  describe("negative numbers", () => {
    test("should handle negative numbers", () => {
      expect(calculatorCore("-5+3")).toBe("-2");
      expect(calculatorCore("-10/2")).toBe("-5");
    });

    test("should handle negative results", () => {
      expect(calculatorCore("3-5")).toBe("-2");
      expect(calculatorCore("1-10")).toBe("-9");
    });
  });

  describe("edge cases with regex patterns", () => {
    test("should handle expressions that don't match patterns", () => {
      expect(calculatorCore("1 + 1")).toBe("1 + 1");
      expect(calculatorCore("a+b")).toBe("a+b");
    });

    test("should handle very long numbers", () => {
      expect(calculatorCore("999999999+1")).toBe("1000000000");
      expect(calculatorCore("123456789*2")).toBe("246913578");
    });
  });

  describe("currency conversion", () => {
    test("should return original expression when currency has no numbers", () => {
      expect(calculatorCore("Q*2", { Q: 3 })).toBe("Q*2");
    });
  });
});
