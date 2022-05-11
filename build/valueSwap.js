"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const valueSwap = (x, y) => {
    let tmp;
    if (y < x) {
        tmp = y;
        y = x;
        x = tmp;
    }
    return [x, y];
};
exports.default = valueSwap;
