export interface TOBINARY {
    (): (x: number, radix?: number) => string;
    (x: number, radix?: number): string;
}
/**
 * n進数に変換
 * @param {number} x
 * @param  {number} [radix=2] n進数
 */
export const toBinary = ((x: number, radix = 2) => {
    if (typeof x === 'undefined') {
        return (x: number, radix = 2) => toBinary(x, radix);
    }
    return x.toString(radix);
}) as TOBINARY;
