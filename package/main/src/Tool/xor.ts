import { XOR } from '../types/logicType';

const xor = ((x: boolean, y: boolean) => {
    return x !== y;
}) as <X extends boolean, Y extends boolean>(x: X, y: Y) => XOR<X, Y>;
export default xor;
