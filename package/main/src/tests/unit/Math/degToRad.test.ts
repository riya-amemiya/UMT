import { degToRad } from "@/Math/degToRad";

describe("degToRad", () => {
  // 正常ケース
  test("degToRad: Normal Cases", () => {
    expect(degToRad(0)).toBeCloseTo(0);
    expect(degToRad(90)).toBeCloseTo(Math.PI / 2);
    expect(degToRad(180)).toBeCloseTo(Math.PI);
    expect(degToRad(270)).toBeCloseTo((3 * Math.PI) / 2);
    expect(degToRad(360)).toBeCloseTo(2 * Math.PI);
  });

  // 負の角度
  test("degToRad: Negative Angles", () => {
    expect(degToRad(-90)).toBeCloseTo(-Math.PI / 2);
    expect(degToRad(-180)).toBeCloseTo(-Math.PI);
    expect(degToRad(-270)).toBeCloseTo((-3 * Math.PI) / 2);
    expect(degToRad(-360)).toBeCloseTo(-2 * Math.PI);
  });

  // 浮動小数点数の角度
  test("degToRad: Floating Point Angles", () => {
    expect(degToRad(45.5)).toBeCloseTo(Math.PI / 4 + Math.PI / 360);
    expect(degToRad(60.7)).toBeCloseTo((Math.PI * 60.7) / 180);
    expect(degToRad(120.2)).toBeCloseTo((Math.PI * 120.2) / 180);
  });

  // 360度以上と1000度以上の角度
  test("degToRad: Angles Greater Than 360 and 1000 Degrees", () => {
    expect(degToRad(450)).toBeCloseTo((5 * Math.PI) / 2);
    expect(degToRad(720)).toBeCloseTo(4 * Math.PI);
    expect(degToRad(1080)).toBeCloseTo(6 * Math.PI);
    expect(degToRad(1440)).toBeCloseTo(8 * Math.PI);
  });

  // NaNとInfinity
  test("degToRad: NaN and Infinity", () => {
    expect(degToRad(NaN)).toBe(NaN);
    expect(degToRad(Infinity)).toBe(Infinity);
    expect(degToRad(-Infinity)).toBe(-Infinity);
  });

  // 無効な入力
  test("degToRad: Invalid Inputs", () => {
    expect(degToRad("90" as any)).toBeCloseTo(Math.PI / 2);
    expect(degToRad([90] as any)).toBeCloseTo(Math.PI / 2);
    expect(degToRad({} as any)).toBe(NaN);
  });
});
