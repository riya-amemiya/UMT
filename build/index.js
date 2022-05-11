"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.getDecimalLength = exports.EuclideanAlgorithm = exports.ValueSwap = void 0;
const ValueSwap_1 = __importDefault(require("./ValueSwap"));
exports.ValueSwap = ValueSwap_1.default;
const EuclideanAlgorithm_1 = __importDefault(require("./EuclideanAlgorithm"));
exports.EuclideanAlgorithm = EuclideanAlgorithm_1.default;
const getDecimalLength_1 = __importDefault(require("./getDecimalLength"));
exports.getDecimalLength = getDecimalLength_1.default;
const UMT = {
    ValueSwap: ValueSwap_1.default,
    EuclideanAlgorithm: EuclideanAlgorithm_1.default,
    getDecimalLength: getDecimalLength_1.default,
};
exports.default = UMT;
