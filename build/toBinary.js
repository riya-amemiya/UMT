"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const toBinary = (x, radix = 2) => {
    if (typeof x === 'undefined') {
        return (x, radix = 2) => toBinary(x, radix);
    }
    return x.toString(radix);
};
exports.default = toBinary;
