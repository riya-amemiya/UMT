/**
 * ラジアンを角度に変換
 * @param  {number} x
 * @returns number
 * @example radToDeg(Math.PI); // 180
 */
export const radToDeg = (x: number) => x / (Math.PI / 180);
