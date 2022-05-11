"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const max = (...num) => Math.max.apply(null, [...new Set(num)]);
exports.default = max;
