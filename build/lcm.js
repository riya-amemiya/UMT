"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const euclideanAlgorithm_1 = __importDefault(require("./euclideanAlgorithm"));
const valueSwap_1 = __importDefault(require("./valueSwap"));
const lcm = (x, y) => {
    if (x === 0 || y === 0) {
        return 0;
    }
    [x, y] = (0, valueSwap_1.default)(x, y);
    return (x / (0, euclideanAlgorithm_1.default)(x, y)) * y;
};
exports.default = lcm;
