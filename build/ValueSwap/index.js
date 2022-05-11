"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const ValueSwap = (x, y) => {
    let tmp;
    if (y < x) {
        tmp = y;
        y = x;
        x = tmp;
    }
    return [x, y];
};
exports.default = ValueSwap;
