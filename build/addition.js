"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const getDecimalLength_1 = __importDefault(require("./getDecimalLength"));
const max_1 = __importDefault(require("./max"));
const multiplication_1 = __importDefault(require("./multiplication"));
const addition = (x, y) => {
    const z = Math.pow(10, (0, max_1.default)((0, getDecimalLength_1.default)(x), (0, getDecimalLength_1.default)(y)));
    return ((0, multiplication_1.default)(x, z) + (0, multiplication_1.default)(y, z)) / z;
};
exports.default = addition;
