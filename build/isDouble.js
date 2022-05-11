"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const isDouble = (x, loose = true) => {
    if (loose) {
        return (isFinite(x) &&
            !Number.isNaN(x) &&
            Number.isFinite(Number(x)) &&
            !Number.isInteger(Number(x)));
    }
    else {
        return (x !== null && typeof x !== 'boolean' && Number.isFinite(x));
    }
};
exports.default = isDouble;
