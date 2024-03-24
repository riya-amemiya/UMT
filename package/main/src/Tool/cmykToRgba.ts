import { division } from "@/Math/division";
import { max } from "@/Math/max";
import { min } from "@/Math/min";
import { multiplication } from "@/Math/multiplication";
import { roundOf } from "@/Math/roundOf";
import { subtract } from "@/Math/subtract";

/**
 * cmykをrgbaに変換する
 * @param cmyk
 * @returns { r: number; g: number; b: number; a: number; }
 * @example cmykToRgba(100, 100, 0, 60.78) // { r: 0, g: 0, b: 100, a: 1 }
 */
export const cmykToRgba = (
  c: number,
  m: number,
  y: number,
  k: number,
  a = 1,
): { r: number; g: number; b: number; a: number } => {
  // CMYKの値を0から100の範囲に制限
  const clampedC = max(0, min(100, c));
  const clampedM = max(0, min(100, m));
  const clampedY = max(0, min(100, y));
  const clampedK = max(0, min(100, k));

  // CMYKの値を0から1の範囲に変換
  const cPercentage = division(clampedC, 100);
  const mPercentage = division(clampedM, 100);
  const yPercentage = division(clampedY, 100);
  const kPercentage = division(clampedK, 100);

  // RGBの値を計算
  const r = multiplication(
    255,
    subtract(1, cPercentage),
    subtract(1, kPercentage),
  );
  const g = multiplication(
    255,
    subtract(1, mPercentage),
    subtract(1, kPercentage),
  );
  const b = multiplication(
    255,
    subtract(1, yPercentage),
    subtract(1, kPercentage),
  );

  // RGBの値を0から255の範囲に丸める
  const roundedR = roundOf(r);
  const roundedG = roundOf(g);
  const roundedB = roundOf(b);

  // アルファ値を0から1の範囲に制限
  const clampedA = max(0, min(1, a));

  return {
    r: roundedR,
    g: roundedG,
    b: roundedB,
    a: clampedA,
  };
};
