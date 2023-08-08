import { calculator } from "@/Math/calculator";
test("calculator", () => {
  expect(calculator("1+1")).toBe("2");
  expect(calculator("1+1+1")).toBe("3");
  expect(calculator("1+1+1+1")).toBe("4");
  expect(calculator("1+1+1+1+1")).toBe("5");
  expect(calculator("2*2")).toBe("4");
  expect(calculator("2*2*2")).toBe("8");
  expect(calculator("2*2*2*2")).toBe("16");
  expect(calculator("2*2*2*2*2")).toBe("32");
  expect(calculator("(1+1)")).toBe("2");
  expect(calculator("(1+1)+(1+1)")).toBe("4");
  expect(calculator("2x=(1+1)+(1+1)+(1+1)")).toBe("3/1");
  expect(calculator("$10*2", { $: 100 })).toBe("2000");
});
