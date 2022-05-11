"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const isNumber = (x, loose = true) => {
    return x !== null && typeof x !== 'boolean' && loose
        ? isFinite(x)
        : Number.isFinite(x);
};
exports.default = isNumber;
