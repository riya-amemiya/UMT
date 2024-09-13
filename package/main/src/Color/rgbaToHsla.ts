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
export const rgbaToHsla = ({
  r,
  g,
  b,
  a = 1,
}: {
  r: number;
  g: number;
  b: number;
  a?: number;
}): { h: number; s: number; l: number; a: number } => {
  const rPrime = division(r, 255);
  const gPrime = division(g, 255);
  const bPrime = division(b, 255);

  const max = Math.max(rPrime, gPrime, bPrime);
  const min = Math.min(rPrime, gPrime, bPrime);
  const diff = subtract(max, min);

  let h = 0;
  const l = division(addition(max, min), 2);
  const s =
    diff === 0
      ? 0
      : division(diff, subtract(1, Math.abs(subtract(addition(max, min), 1))));

  if (diff !== 0) {
    // biome-ignore lint/style/useDefaultSwitchClause: <explanation>
    switch (max) {
      case rPrime: {
        h = addition(
          division(subtract(gPrime, bPrime), diff),
          gPrime < bPrime ? 6 : 0,
        );
        break;
      }
      case gPrime: {
        h = addition(division(subtract(bPrime, rPrime), diff), 2);
        break;
      }
      case bPrime: {
        h = addition(division(subtract(rPrime, gPrime), diff), 4);
        break;
      }
    }
    h = multiplication(h, 60);
  }

  return {
    h: roundOf(h, 2),
    s: roundOf(s * 100, 2),
    l: roundOf(l * 100, 2),
    a,
  };
};
