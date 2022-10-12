import { isBoolean, OR } from '../types/logicType';

export const or = (<X, Y>(x: X, y: Y) => {
    return x || y;
}) as <X, Y>(x: X, y: Y) => OR<isBoolean<X>, isBoolean<Y>>;
