import { isBoolean, XOR } from '../types/logicType';

export const xor = ((x: boolean, y: boolean) => {
    return x !== y;
}) as <X, Y>(x: X, y: Y) => XOR<isBoolean<X>, isBoolean<Y>>;
