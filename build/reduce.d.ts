declare const reduce: (x: number, y: number) => {
    x: number;
    y: number;
    gcd?: undefined;
} | {
    x: number;
    y: number;
    gcd: number;
};
export default reduce;
