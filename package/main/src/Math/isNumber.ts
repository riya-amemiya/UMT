/**
 * 数字かどうか
 * @param  {any} x
 * @param  {boolean} loose 曖昧な判定を許すかどうか(default:true)
 */
export const isNumber = (x: any, loose: boolean = true) => {
    return x !== null && typeof x !== 'boolean' && loose
        ? isFinite(x)
        : Number.isFinite(x);
};
