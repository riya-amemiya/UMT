import { calculator } from "@/Math/calculator";
test("calculator", () => {
  // 足し算
  expect(calculator("1+1")).toBe("2");
  expect(calculator("1+1+1")).toBe("3");
  expect(calculator("1+1+1+1")).toBe("4");
  expect(calculator("1+1+1+1+1")).toBe("5");

  // 引き算
  expect(calculator("1-1")).toBe("0");

  // 掛け算
  expect(calculator("2*2")).toBe("4");
  expect(calculator("2*2*2")).toBe("8");

  // 割り算
  expect(calculator("2/2")).toBe("1");
  expect(calculator("2/2/2")).toBe("0.5");

  // 括弧
  expect(calculator("(1+1)")).toBe("2");
  expect(calculator("(1+1)+(1+1)")).toBe("4");

  // 方程式
  expect(calculator("2x=(1+1)+(1+1)+(1+1)")).toBe("3/1");

  // 変数
  expect(calculator("$10*2", { $: 100 })).toBe("2000");
});
