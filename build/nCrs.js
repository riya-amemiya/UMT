"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const nCr_1 = __importDefault(require("./nCr"));
const nCrs = (n, r) => {
    if (n === 0 || r === 0) {
        return NaN;
    }
    const y = (0, nCr_1.default)(n + r - 1, r);
    if (1 > y)
        return 0;
    return y;
};
exports.default = nCrs;