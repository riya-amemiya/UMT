"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const primeFactorization = (x) => {
    let n = 0;
    const out = [];
    for (let i = 2; i <= x; i++) {
        if (x % i === 0) {
            n = 0;
            while (x % i === 0) {
                n++;
                x /= i;
            }
            out.push({ number: i, count: n });
        }
    }
    return out;
};
exports.default = primeFactorization;
