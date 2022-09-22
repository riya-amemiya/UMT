const compilerCore = require('./module/Compiler/core').default;
const tokenList = [
    ['PRINT', /print/, 6],
    ['STRING', /"[^"]*"/, 1],
    ['OPEN_PARENTHESIS', /\(/, 1],
    ['CLOSE_PARENTHESIS', /\)/, 1],
    ['INT', /\d+/, 4],
    ['PLUS', /\+/, 3],
    ['MINUS', /-/, 3],
    ['MULTIPLY', /\*/, 2],
    ['DIVIDE', /\//, 2],
    ['COMMENT', /#.*\n/, null],
    ['EQUAL', /=/, 3],
    ["NAME",/^(?!.*").+$/,0]
];
const process = (_code, _tokenList, x) => {
    if (x.name === 'PRINT') {
        return `puts`;
    } else {
        return x.value;
    }
};
console.log('====================================');
console.log(
    compilerCore(
        `print("Hello")\nprint(10 + 20 * 30)\nage = 10`,
        tokenList,
        process,
    ),
);
console.log('====================================');
