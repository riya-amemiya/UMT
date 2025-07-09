/**
 * Options for isDeepEqual comparison
 */
export interface IsDeepEqualOptions {
  /**
   * Whether to ignore array order when comparing arrays
   * @default true
   */
  strictOrder?: boolean;
}

/**
 * Performs a deep equality comparison between two values
 *
 * @param a - First value to compare
 * @param b - Second value to compare
 * @param options - Comparison options
 * @returns true if values are deeply equal, false otherwise
 *
 * @example
 * ```typescript
 * isDeepEqual({ a: 1, b: [2, 3] }, { b: [2, 3], a: 1 }); // true
 * isDeepEqual([1, 2, 3], [3, 2, 1]); // false
 * isDeepEqual([1, 2, 3], [3, 2, 1], { strictOrder: false }); // true
 * isDeepEqual(new Set([1, 2]), new Set([2, 1])); // true
 * isDeepEqual(new Map([['a', 1]]), new Map([['a', 1]])); // true
 * ```
 */
export function isDeepEqual(
  a: unknown,
  b: unknown,
  options: IsDeepEqualOptions = {},
): boolean {
  const { strictOrder = true } = options;
  const stack = [{ x: a, y: b }];

  while (stack.length > 0) {
    // biome-ignore lint/style/noNonNullAssertion: stack.length > 0 ensures pop() returns a value
    const current = stack.pop()!;

    const { x, y } = current;

    if (Object.is(x, y)) {
      continue;
    }

    if (x == null || y == null) {
      return false;
    }

    if (typeof x !== typeof y) {
      return false;
    }

    if (typeof x !== "object" || typeof y !== "object") {
      return false;
    }

    const objectX = x;
    const objectY = y;

    const ctorX = objectX.constructor;
    const ctorY = objectY.constructor;
    if (ctorX !== ctorY) {
      return false;
    }

    if (x instanceof Date && y instanceof Date) {
      if (x.getTime() !== y.getTime()) {
        return false;
      }
      continue;
    }

    if (x instanceof RegExp && y instanceof RegExp) {
      if (x.toString() !== y.toString()) {
        return false;
      }
      continue;
    }

    if (Array.isArray(x) && Array.isArray(y)) {
      if (x.length !== y.length) {
        return false;
      }

      if (strictOrder) {
        for (let index = x.length - 1; index >= 0; index--) {
          stack.push({ x: x[index], y: y[index] });
        }
      } else {
        return false;
      }
      continue;
    }

    if (x instanceof Set && y instanceof Set) {
      if (x.size !== y.size) {
        return false;
      }

      const arrayX = Array.from(x);
      const arrayY = Array.from(y);

      for (const itemX of arrayX) {
        let found = false;
        for (let index = 0; index < arrayY.length; index++) {
          const itemY = arrayY[index];
          if (Object.is(itemX, itemY)) {
            arrayY.splice(index, 1);
            found = true;
            break;
          }
        }
        if (!found) {
          return false;
        }
      }
      continue;
    }

    if (x instanceof Map && y instanceof Map) {
      if (x.size !== y.size) {
        return false;
      }

      const entriesX = Array.from(x.entries());
      const entriesY = Array.from(y.entries());

      for (const [keyX, valueX] of entriesX) {
        let found = false;
        for (let index = 0; index < entriesY.length; index++) {
          const [keyY, valueY] = entriesY[index];
          if (Object.is(keyX, keyY) && Object.is(valueX, valueY)) {
            entriesY.splice(index, 1);
            found = true;
            break;
          }
        }
        if (!found) {
          return false;
        }
      }
      continue;
    }

    if (ArrayBuffer.isView(x) && ArrayBuffer.isView(y)) {
      const xArray = x as Uint8Array;
      const yArray = y as Uint8Array;
      if (xArray.byteLength !== yArray.byteLength) {
        return false;
      }
      for (let index = 0; index < xArray.byteLength; index++) {
        if (xArray[index] !== yArray[index]) {
          return false;
        }
      }
      continue;
    }

    const keysX = Object.keys(objectX);
    const keysY = Object.keys(objectY);

    if (keysX.length !== keysY.length) {
      return false;
    }

    for (const key of keysX) {
      if (!(key in objectY)) {
        return false;
      }
    }

    for (let index = keysX.length - 1; index >= 0; index--) {
      const key = keysX[index];
      stack.push({
        x: objectX[key as keyof typeof objectX],
        y: objectY[key as keyof typeof objectY],
      });
    }
  }

  return true;
}
