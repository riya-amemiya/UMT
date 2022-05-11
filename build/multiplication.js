"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const getDecimalLength_1 = __importDefault(require("./getDecimalLength"));
const multiplication = (x, y) => {
    const n = Math.pow(10, ((0, getDecimalLength_1.default)(x) + (0, getDecimalLength_1.default)(y)));
    x = +(x + '').replace('.', '');
    y = +(y + '').replace('.', '');
    return (x * y) / n;
};
exports.default = multiplication;
