import { OR } from '../types/logicType';

const or = (<X, Y>(x: X, y: Y) => {
    return x || y;
}) as <X extends boolean, Y extends boolean>(x: X, y: Y) => OR<X, Y>;
export default or;
