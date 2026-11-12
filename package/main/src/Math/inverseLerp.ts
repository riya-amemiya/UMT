/**
Calculates the interpolation factor of a value between two values
@param {number} start Value that maps to 0
@param {number} end Value that maps to 1
@param {number} value Value to locate; values outside the range give factors outside 0..1
@returns {number} (value - start) / (end - start), or 0 when start equals end
@example inverseLerp(0, 10, 5); // 0.5
@example inverseLerp(10, 20, 12.5); // 0.25
@example inverseLerp(5, 5, 7); // 0
*/
export const inverseLerp = (
  start: number,
  end: number,
  value: number,
): number => (start === end ? 0 : (value - start) / (end - start));
