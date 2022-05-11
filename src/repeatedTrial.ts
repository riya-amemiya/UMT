import reduce from './reduce';
/**
 * 反復的な思考
 * @param  {number} n
 * @param  {number} r
 * @param  {{x:number;y:number}} p
 */
const repeatedTrial = (
    n: number,
    r: number,
    p: { x: number; y: number },
) => {
    const x = [p.x ** r, p.y ** r];
    const y = [(p.y - p.x) ** (n - r), p.y ** (n - r)];
    p.x = x[0] * y[0] * 10;
    p.y = x[1] * y[1];
    const z = reduce(p.x, p.y);
    return [z.x, z.y];
};
export default repeatedTrial;
