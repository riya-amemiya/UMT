import { random } from "@/Math/random";

describe("random関数のテスト", () => {
  it("指定された範囲内の乱数を生成する", () => {
    const min = 1;
    const max = 10;
    const result = random(max, min);
    expect(result).toBeGreaterThanOrEqual(min);
    expect(result).toBeLessThanOrEqual(max);
  });

  it("最小値を指定しない場合は0を含む乱数を生成する", () => {
    const max = 5;
    const result = random(max);
    expect(result).toBeGreaterThanOrEqual(0);
    expect(result).toBeLessThanOrEqual(max);
  });

  it("最大値と最小値が同じ場合はその値を返す", () => {
    const value = 7;
    const result = random(value, value);
    expect(result).toBe(value);
  });
});
