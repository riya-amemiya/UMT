import { npr } from "@/umt_plugin_wasm";
describe("nPr function", () => {
  // 正常系
  test("calculate nPr for n=5, r=2", () => {
    expect(npr(5, 2)).toBe(20);
  });

  test("calculate nPr for n=10, r=4", () => {
    expect(npr(10, 4)).toBe(5040);
  });
});
