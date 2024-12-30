import { radToDeg } from "@/Math/radToDeg";

describe("radToDeg", () => {
  // 正常ケース
  test("radToDeg: Normal Cases", () => {
    expect(radToDeg(0)).toBe(0);
    expect(radToDeg(Math.PI / 2)).toBe(90);
    expect(radToDeg(Math.PI)).toBe(180);
    expect(radToDeg((3 * Math.PI) / 2)).toBe(270);
    expect(radToDeg(2 * Math.PI)).toBe(360);
  });

  // 負の角度
  test("radToDeg: Negative Angles", () => {
    expect(radToDeg(-Math.PI / 2)).toBe(-90);
    expect(radToDeg(-Math.PI)).toBe(-180);
    expect(radToDeg((-3 * Math.PI) / 2)).toBe(-270);
    expect(radToDeg(-2 * Math.PI)).toBe(-360);
  });

  // 浮動小数点数の角度
  test("radToDeg: Floating Point Angles", () => {
    expect(radToDeg(Math.PI / 4)).toBeCloseTo(45);
    expect(radToDeg((Math.PI * 60.7) / 180)).toBeCloseTo(60.7);
    expect(radToDeg((Math.PI * 120.2) / 180)).toBeCloseTo(120.2);
  });

  // 2π以上の角度
  test("radToDeg: Angles Greater Than 2π", () => {
    expect(radToDeg((5 * Math.PI) / 2)).toBeCloseTo(450);
    expect(radToDeg(4 * Math.PI)).toBeCloseTo(720);
    expect(radToDeg(6 * Math.PI)).toBeCloseTo(1080);
  });

  // NaNとInfinity
  test("radToDeg: NaN and Infinity", () => {
    expect(radToDeg(NaN)).toBe(NaN);
    expect(radToDeg(Infinity)).toBe(Infinity);
    expect(radToDeg(-Infinity)).toBe(-Infinity);
  });

  // 無効な入力
  test("radToDeg: Invalid Inputs", () => {
    expect(radToDeg("1.57" as any)).toBeCloseTo(89.95);
    expect(radToDeg([1.57] as any)).toBeCloseTo(89.95);
    expect(radToDeg({} as any)).toBe(NaN);
  });
});
