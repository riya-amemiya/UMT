"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const getArraysDiff = (array, ...arrays) => {
    let hasArr = [];
    for (const i of arrays) {
        hasArr.push(...i);
    }
    const arr01 = [...new Set(array)];
    const arr02 = [...new Set(hasArr)];
    return [...arr01, ...arr02].filter((value) => !arr01.includes(value) || !arr02.includes(value));
};
exports.default = getArraysDiff;