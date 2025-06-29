import { literalExpression } from "@/Math/calculator/literalExpression";

describe("literalExpression function", () => {
  // Basic equations with variable on left side
  test("should handle basic equations", () => {
    expect(literalExpression("x=1")).toBe("1");
    expect(literalExpression("x=0")).toBe("0");
    expect(literalExpression("x=-1")).toBe("-1");
  });

  // Basic equations with variable on right side
  test("should handle basic equations (right side)", () => {
    expect(literalExpression("1=x")).toBe("1");
    expect(literalExpression("0=x")).toBe("0");
    expect(literalExpression("-1=x")).toBe("-1");
  });

  // Equations with coefficients and additional terms (variable on left)
  test("should handle equations with mixed numerical and variable parts", () => {
    expect(literalExpression("2x=4")).toBe("2");
    expect(literalExpression("1x=1")).toBe("1");
    expect(literalExpression("3x=-6")).toBe("-2");
    expect(literalExpression("2x+2=4")).toBe("1");
    expect(literalExpression("3x-2=-6")).toBe("-4/3");
  });

  // Equations with coefficients and additional terms (variable on right)
  test("should handle equations with mixed numerical and variable parts (right side)", () => {
    expect(literalExpression("4=2x")).toBe("2");
    expect(literalExpression("1=1x")).toBe("1");
    expect(literalExpression("-6=3x")).toBe("-2");
    expect(literalExpression("4=2x+2")).toBe("1");
    expect(literalExpression("-6=3x-2")).toBe("-4/3");
  });

  // Simplification using greatest common divisor (variable on left)
  test("should simplify when gcd is greater than 1", () => {
    expect(literalExpression("2x=8")).toBe("4");
  });

  // Simplification using greatest common divisor (variable on right)
  test("should simplify when gcd is greater than 1 (right side)", () => {
    expect(literalExpression("8=2x")).toBe("4");
  });

  test("should handle fractions that cannot be simplified to denominator 1", () => {
    expect(literalExpression("6x=4")).toBe("2/3");
  });

  // Invalid equations
  test("should handle invalid equations", () => {
    expect(literalExpression("x=x")).toBe("");
  });
});
