"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const ValueSwap_1 = __importDefault(require("../ValueSwap"));
const EuclideanAlgorithm = (x, y, ...z) => {
    if (x === 0 || y === 0)
        return 0;
    [x, y] = (0, ValueSwap_1.default)(x, y);
    let r = y % x;
    while (r !== 0) {
        y = x;
        x = r;
        r = y % x;
    }
    if (z.length > 0) {
        for (let i = 0; i < z.length; i++) {
            x = EuclideanAlgorithm(x, z[i]);
        }
    }
    return x;
};
exports.default = EuclideanAlgorithm;
