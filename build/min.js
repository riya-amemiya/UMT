"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const min = (num) => Math.min.apply(null, [...new Set(num)]);
exports.default = min;
