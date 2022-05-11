"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const division_1 = __importDefault(require("./division"));
const isPrimeNumber = (n) => {
    if (n < 2)
        return false;
    else if (n == 2)
        return true;
    else if (n % 2 == 0)
        return false;
    for (let i = 3; i <= Math.sqrt(n); i++) {
        if ((0, division_1.default)(n, i)[1] == 0) {
            return false;
        }
    }
    return true;
};
exports.default = isPrimeNumber;