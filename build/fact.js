"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const fact = (x) => {
    if (x === 0) {
        return 1;
    }
    else {
        return x * fact(x - 1);
    }
};
exports.default = fact;
