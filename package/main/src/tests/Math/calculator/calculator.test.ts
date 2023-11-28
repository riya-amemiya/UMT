import { calculator } from "@/Math/calculator";
describe("calculator", () => {
  // 足し算
  test("should handle addition", () => {
    expect(calculator("1+")).toBe("1+");
    expect(calculator("1+1")).toBe("2");
    expect(calculator("1+1+1")).toBe("3");
    expect(calculator("1+1+1+1")).toBe("4");
    expect(calculator("1+1+1+1+1")).toBe("5");
  });

  // 引き算
  test("should handle subtraction", () => {
    expect(calculator("1-")).toBe("1-");
    expect(calculator("1-1")).toBe("0");
    expect(calculator("1-1-1")).toBe("-1");
    expect(calculator("1-1-1-1")).toBe("-2");
    expect(calculator("1-1-1-1-1")).toBe("-3");
  });

  // 掛け算
  test("should handle multiplication", () => {
    expect(calculator("2*")).toBe("2*");
    expect(calculator("2*2")).toBe("4");
    expect(calculator("2*2*2")).toBe("8");
    expect(calculator("2*2*2*2")).toBe("16");
    expect(calculator("2*2*2*2*2")).toBe("32");
  });

  // 割り算
  test("should handle division", () => {
    expect(calculator("2/")).toBe("2/");
    expect(calculator("2/2")).toBe("1");
    expect(calculator("2/2/2")).toBe("0.5");
    expect(calculator("2/2/2/2")).toBe("0.25");
    expect(calculator("2/2/2/2/2")).toBe("0.125");
  });

  // べき乗
  test("should handle exponentiation", () => {
    expect(calculator("2^")).toBe("2^");
    expect(calculator("2^2")).toBe("4");
    expect(calculator("2^2^2")).toBe("16");
    expect(calculator("2^2^2^2")).toBe("65536");
    expect(calculator("2^2^2^2^2")).toBe("Infinity");
    expect(calculator("3^4")).toBe("81");
    expect(calculator("3^4^2")).toBe("43046721");
  });

  // 括弧
  test("should handle parentheses", () => {
    expect(calculator("(1+1)")).toBe("2");
    expect(calculator("(1")).toBe("(1");
    expect(calculator("(1+")).toBe("(1+");
    expect(calculator("1)")).toBe("1)");
    expect(calculator("(1)")).toBe("(1)");
    expect(calculator("(1+1)+(1+1)")).toBe("4");
  });

  // 方程式
  test("should handle equations", () => {
    expect(calculator("2x=(1+1)+(1+1)+(1+1)")).toBe("3");
  });

  // 変数
  test("should handle variables", () => {
    expect(calculator("$10*2", { $: 100 })).toBe("2000");
    expect(calculator("2*$10", { $: 100 })).toBe("2000");
  });
});
