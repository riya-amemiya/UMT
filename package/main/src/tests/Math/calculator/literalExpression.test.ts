import { literalExpression } from "@/Math/calculator/literalExpression";
describe("literalExpression", () => {
  // 基本的な方程式
  test("should handle basic equations", () => {
    expect(literalExpression("x=1")).toBe("1");
    expect(literalExpression("x=0")).toBe("0");
    expect(literalExpression("x=-1")).toBe("-1");
  });

  // 基本的な方程式(右辺)
  test("should handle basic equations (right side)", () => {
    expect(literalExpression("1=x")).toBe("1");
    expect(literalExpression("0=x")).toBe("0");
    expect(literalExpression("-1=x")).toBe("-1");
  });

  // 数値と変数が混在
  test("should handle equations with mixed numerical and variable parts", () => {
    expect(literalExpression("2x=4")).toBe("2");
    expect(literalExpression("3x=-6")).toBe("-2");
  });

  // 数値と変数が混在(右辺)
  test("should handle equations with mixed numerical and variable parts (right side)", () => {
    expect(literalExpression("4=2x")).toBe("2");
    expect(literalExpression("-6=3x")).toBe("-2");
  });

  // 最大公約数が1より大きい場合
  test("should simplify when gcd is greater than 1", () => {
    expect(literalExpression("2x=8")).toBe("4");
  });
});
