import { AND } from '../types/logicType';

const and = (<X extends boolean, Y extends boolean>(x: X, y: Y) => {
    return x && y;
}) as <X extends boolean, Y extends boolean>(x: X, y: Y) => AND<X, Y>;

export { and };
