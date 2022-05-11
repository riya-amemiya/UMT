"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const getDecimalLength = (value) => {
    let x = (value + '').split('.')[1];
    if (typeof x !== 'undefined' && x.length > 0) {
        return x.length;
    }
    return 0;
};
exports.default = getDecimalLength;
