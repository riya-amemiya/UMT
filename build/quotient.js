"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const quotient = (x, y) => [
    (x - (x % y)) / y,
    (x % y) + 0,
];
exports.default = quotient;
