"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const nPr_1 = __importDefault(require("./nPr"));
const nCr = (n, r) => {
    let y = (0, nPr_1.default)(n, r);
    let age = 1;
    for (let i = 2; i <= r; i++) {
        age *= i;
    }
    y /= age;
    if (1 > y)
        return 0;
    return y;
};
exports.default = nCr;
