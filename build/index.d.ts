import ValueSwap from './ValueSwap';
import EuclideanAlgorithm from './EuclideanAlgorithm';
import getDecimalLength from './getDecimalLength';
declare const UMT: {
    ValueSwap: (x: number, y: number) => number[];
    EuclideanAlgorithm: (x: number, y: number, ...z: number[]) => number;
    getDecimalLength: (value: number) => number;
};
export { ValueSwap, EuclideanAlgorithm, getDecimalLength };
export default UMT;
