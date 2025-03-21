import { addition } from "@/Math/addition";
import { division } from "@/Math/division";
import { multiplication } from "@/Math/multiplication";
import { roundOf } from "@/Math/roundOf";
import { subtract } from "@/Math/subtract";

/**
 * Convert RGBA color values to HSLA color space
 * @param rgba Object containing r, g, b values (0-255) and optional a (0-1)
 * @returns {Object} HSLA values (h as 0-360, s and l as 0-100, a as 0-1)
 * @example rgbaToHsla({ r: 100, g: 100, b: 100, a: 1 }); // { h: 0, s: 0, l: 39.22, a: 1 }
 * @throws {Error} If any input values are out of their valid ranges
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
  // Validate RGBA values
  if (
    r < 0 ||
    r > 255 ||
    g < 0 ||
    g > 255 ||
    b < 0 ||
    b > 255 ||
    a < 0 ||
    a > 1
  ) {
    throw new Error("Invalid rgba value");
  }

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
