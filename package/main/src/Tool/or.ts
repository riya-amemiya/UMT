import { OR } from '../types/logicType';

export const or = (<X, Y>(x: X, y: Y) => {
    return x || y;
}) as <X extends boolean, Y extends boolean>(x: X, y: Y) => OR<X, Y>;
