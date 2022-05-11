"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const arraysJoin = (array, ...arrays) => {
    for (const i of arrays) {
        array.push(...i);
    }
    return [...new Set(array)];
};
exports.default = arraysJoin;
