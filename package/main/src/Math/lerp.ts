/**
Linearly interpolates between two values
@param {number} start Value returned when t is 0
@param {number} end Value returned when t is 1
@param {number} t Interpolation factor; values outside 0..1 extrapolate
@returns {number} start + (end - start) * t
@example lerp(0, 10, 0.5); // 5
@example lerp(10, 20, 0.25); // 12.5
@example lerp(0, 10, 2); // 20
*/
export const lerp = (start: number, end: number, t: number): number =>
  start + (end - start) * t;
