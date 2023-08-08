import { exchange } from "@/Math/calculator/exchange";
test("exchange", () => {
  expect(exchange("$1", { $: 100 })).toBe("100");
  expect(exchange("a$1", { $: 100 })).toBe("a$1");
  expect(exchange("a$1", { a$: 100 })).toBe("100");
});
