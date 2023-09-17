import { degToRad } from "@/Math/degToRad";

describe("degToRad", () => {
  // 正常ケース
  test("degToRad: Normal Cases", () => {
    expect(degToRad(0)).toBe(0);
    expect(degToRad(90)).toBe(Math.PI / 2);
    expect(degToRad(180)).toBe(Math.PI);
  });

  // 負の角度
  test("degToRad: Negative Angles", () => {
    expect(degToRad(-90)).toBe(-Math.PI / 2);
  });

  // 浮動小数点数の角度
  test("degToRad: Floating Point Angles", () => {
    expect(degToRad(45.5)).toBeCloseTo(Math.PI / 4 + Math.PI / 360);
  });

  // 360度以上の角度
  test("degToRad: Angles Greater Than 360 Degrees", () => {
    expect(degToRad(360)).toBe(2 * Math.PI);
    expect(degToRad(450)).toBe((5 * Math.PI) / 2);
  });
});
