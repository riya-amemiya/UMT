"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const euclideanAlgorithm_1 = __importDefault(require("./euclideanAlgorithm"));
const reduce = (x, y) => {
    if (x === 0 || y === 0) {
        return { x: NaN, y: NaN };
    }
    let n = (0, euclideanAlgorithm_1.default)(x, y);
    return { x: x / n, y: y / n, gcd: (0, euclideanAlgorithm_1.default)(x, y) };
};
exports.default = reduce;
