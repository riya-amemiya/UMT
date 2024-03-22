import { division } from "@/Math/division";
import { roundOf } from "@/Math/roundOf";
import { subtract } from "@/Math/subtract";

/**
 * rgbaをcmykに変換する
 * @param rgba
 * @returns { c: number, m: number, y: number, k: number, a: number }
 * @example rgbaToCmyk({ r: 0, g: 0, b: 0, a: 1 }); // { c: 0, m: 0, y: 0, k: 100, a: 1 }
 */
export const rgbaToCmyk = ({
  r,
  g,
  b,
  a = 1,
}: {
  r: number;
  g: number;
  b: number;
  a?: number;
}): { c: number; m: number; y: number; k: number; a: number } => {
  const rPrime = division(r, 255);
  const gPrime = division(g, 255);
  const bPrime = division(b, 255);

  const k = subtract(1, Math.max(rPrime, gPrime, bPrime));
  const c = division(subtract(subtract(1, rPrime), k), subtract(1, k)) || 0;
  const m = division(subtract(subtract(1, gPrime), k), subtract(1, k)) || 0;
  const y = division(subtract(subtract(1, bPrime), k), subtract(1, k)) || 0;

  return {
    c: roundOf(c * 100, 2),
    m: roundOf(m * 100, 2),
    y: roundOf(y * 100, 2),
    k: roundOf(k * 100, 2),
    a,
  };
};
