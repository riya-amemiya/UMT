import { addition } from '../Math/addition';
export const sum = (x: number[]) => {
    return x.reduce((a, b) => addition(a, b));
};
