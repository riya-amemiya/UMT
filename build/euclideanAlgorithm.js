"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const valueSwap_1 = __importDefault(require("./valueSwap"));
const euclideanAlgorithm = (x, y, ...z) => {
    if (x === 0 || y === 0)
        return 0;
    [x, y] = (0, valueSwap_1.default)(x, y);
    let r = y % x;
    while (r !== 0) {
        y = x;
        x = r;
        r = y % x;
    }
    if (z.length > 0) {
        for (let i = 0; i < z.length; i++) {
            x = euclideanAlgorithm(x, z[i]);
        }
    }
    return x;
};
exports.default = euclideanAlgorithm;
