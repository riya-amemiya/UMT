"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const getArraysIntersect = (array, ...arrays) => {
    let hasArr = [];
    for (const i of arrays) {
        hasArr.push(...i);
    }
    return [...new Set(array)].filter((value) => hasArr.includes(value));
};
exports.default = getArraysIntersect;
