import compilerCore from './module/Compiler/core';
import compilerToken from './module/Compiler/token';
import * as fs from 'fs';
const check = (file: string) => {
    let hasfaile = false;
    try {
        fs.statSync(file);
        hasfaile = true;
    } catch (err) {
        console.log(err);

        hasfaile = false;
    }
    return hasfaile;
};
const read = (file: string) => {
    return check(file) ? fs.readFileSync(file, 'utf8') : '';
};
const tokenList: [string, RegExp, number | null][] = [
    ['PRINT', /print/, 6],
    ['STRING', /"[^"]*"/, 3],
    ['OPEN_PARENTHESIS', /\(/, 1],
    ['CLOSE_PARENTHESIS', /\)/, 1],
    ['INT', /\d+/, 4],
    ['PLUS', /\+/, 3],
    ['MINUS', /-/, 3],
    ['MULTIPLY', /\*/, 2],
    ['DIVIDE', /\//, 2],
    ['COMMENT', /#.*/, 2],
    ['EQUAL', /=/, 3],
    ['IF', /if/, 6],
    ['ELSE', /else/, 6],
    ['WHILE', /while/, 6],
    ['FOR', /for/, 6],
    ['OPEN_BRACKET', /\{/, 1],
    ['CLOSE_BRACKET', /\}/, 1],
    ['OPEN_BRACKET_SQUARE', /\[/, 1],
    ['CLOSE_BRACKET_SQUARE', /\]/, 1],
    ['COMMA', /,/, 1],
    ['SEMICOLON', /;/, 1],
    ['FUNCTION', /def/, 6],
    ['COLON', /:/, null],
    ['RETURN', /return/, null],
];
const process = (
    _code: string,
    _tokenList: [string, RegExp, number | null][],
    x: { name: string; value: string },
) => {
    if (x.name === 'PRINT') {
        return `puts`;
    }
    if (x.name === 'IDENTIFIER') {
        return `${x.value.replace('let', '').slice(1)}`;
    }

    return x.value;
};
console.log('====================================');
console.log(
    compilerToken(read('./code.py'), tokenList, [
        ['IDENTIFIER', /[a-zA-Z_][a-zA-Z0-9_]*/, 0],
    ]),
);
console.log('====================================');
