import { factorial } from "@/Math/factorial";

describe("factorial", () => {
  test("0の階乗は1を返す", () => {
    expect(factorial(0)).toBe(1);
  });

  test("1の階乗は1を返す", () => {
    expect(factorial(1)).toBe(1);
  });

  test("正の整数の階乗を正しく計算する", () => {
    expect(factorial(2)).toBe(2);
    expect(factorial(3)).toBe(6);
    expect(factorial(4)).toBe(24);
    expect(factorial(5)).toBe(120);
    expect(factorial(6)).toBe(720);
    expect(factorial(7)).toBe(5040);
    expect(factorial(8)).toBe(40320);
    expect(factorial(9)).toBe(362880);
    expect(factorial(10)).toBe(3628800);
  });

  test("負の数を渡すと1を返す", () => {
    expect(factorial(-1)).toBe(1);
    expect(factorial(-5)).toBe(1);
  });
});
