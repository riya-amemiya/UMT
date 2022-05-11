"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const reduce_1 = __importDefault(require("./reduce"));
const repeatedTrial = (n, r, p) => {
    const x = [Math.pow(p.x, r), Math.pow(p.y, r)];
    const y = [Math.pow((p.y - p.x), (n - r)), Math.pow(p.y, (n - r))];
    p.x = x[0] * y[0] * 10;
    p.y = x[1] * y[1];
    const z = (0, reduce_1.default)(p.x, p.y);
    return [z.x, z.y];
};
exports.default = repeatedTrial;
