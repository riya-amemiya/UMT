"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const addition_1 = __importDefault(require("./addition"));
const division_1 = __importDefault(require("./division"));
const multiplication_1 = __importDefault(require("./multiplication"));
const subtract_1 = __importDefault(require("./subtract"));
let n = 0;
const calculator = (x) => {
    if (n === 0) {
        x = x.replace(/\s+/g, '');
        n++;
    }
    x = x.replace(/--/g, '+');
    x = x.replace(/\+\+/g, '+');
    x = x.replace(/\+-/g, '+0-');
    x = x.replace(/\-\+/g, '+0-');
    if (x.indexOf('(') != -1 || x.indexOf(')') != -1) {
        const y = x.match(/\(\d+\.?(\d+)?(\*|\/|\+|\-)\d+\.?(\d+)?\)/);
        if (y) {
            return calculator(x.replace(y[0], calculator(y[0].replace(/\(|\)/g, ''))));
        }
        else {
            return calculator(x.replace(`(${x.slice(x.indexOf('(') + 1, x.indexOf(')'))})`, calculator(x.slice(x.indexOf('(') + 1, x.indexOf(')')))));
        }
    }
    else if (x.indexOf('*') != -1 || x.indexOf('/') != -1) {
        const y = [
            x.match(/\d+\.?(\d+)?(\*|\/)\d+\.?(\d+)?/),
            [''],
        ];
        if (y[0]) {
            y[1] = y[0][0].split(/(\d+\.\d+)|(\d+)/g).filter((n) => {
                return typeof n != 'undefined' && n != '';
            });
            return calculator(x.replace(y[0][0], `${y[1][1] == '*'
                ? (0, multiplication_1.default)(Number(y[1][0]), Number(y[1][2]))
                : y[1][1] == '/'
                    ? (0, division_1.default)(Number(y[1][0]), Number(y[1][2]))[0]
                    : '0'}`));
        }
        return x;
    }
    else if (x.indexOf('+') != -1 || x.indexOf('-') != -1) {
        let y = [
            x.match(/\d+\.?(\d+)?(\+|\-)\d+\.?(\d+)?/),
            [''],
        ];
        if (y[0]) {
            y[1] = y[0][0].split(/(\d+\.\d+)|(\d+)/g).filter((n) => {
                return typeof n != 'undefined' && n !== '';
            });
            return calculator(x.replace(y[0][0], `${y[1][1] == '+'
                ? (0, addition_1.default)(Number(y[1][0]), Number(y[1][2]))
                : y[1][1] == '-'
                    ? (0, subtract_1.default)(Number(y[1][0]), Number(y[1][2]))
                    : '0'}`));
        }
        return x;
    }
    else {
        return x;
    }
};
exports.default = calculator;
