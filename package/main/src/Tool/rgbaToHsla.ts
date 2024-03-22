import { addition } from "@/Math/addition";
import { division } from "@/Math/division";
import { multiplication } from "@/Math/multiplication";
import { roundOf } from "@/Math/roundOf";
import { subtract } from "@/Math/subtract";

/**
 * rgbaをhslaに変換する
 * @param rgba
 * @returns { h: number; s: number; l: number; a: number; }
 * @example rgbaToHsla({ r: 100, g: 100, b: 100, a: 1 }); // { h: 0, s: 0, l: 39.22, a: 1 }
 */
export const rgbaToHsla = (rgba: {
  r: number;
  g: number;
  b: number;
  a?: number;
}) => {
  const r = division(rgba.r, 255);
  const g = division(rgba.g, 255);
  const b = division(rgba.b, 255);

  const max = Math.max(r, g, b);
  const min = Math.min(r, g, b);
  const diff = subtract(max, min);

  let h = 0;
  const l = division(addition(max, min), 2);
  const s =
    diff === 0
      ? 0
      : division(diff, subtract(1, Math.abs(subtract(addition(max, min), 1))));

  if (diff !== 0) {
    switch (max) {
      case r: {
        h = addition(division(subtract(g, b), diff), g < b ? 6 : 0);
        break;
      }
      case g: {
        h = addition(division(subtract(b, r), diff), 2);
        break;
      }
      case b: {
        h = addition(division(subtract(r, g), diff), 4);
        break;
      }
    }
    h = multiplication(h, 60);
  }

  return {
    h: roundOf(h, 2),
    s: roundOf(s * 100, 2),
    l: roundOf(l * 100, 2),
    a: rgba.a || 1,
  };
};
