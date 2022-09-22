const compilerCore = require('./module/Compiler/core').default;
const tokenList = [
    ['PRINT', /print/g, 5],
    ['STRING', /"[^"]*"/g, 4],
    ['OPEN_PARENTHESIS', /\(/g, 0],
    ['CLOSE_PARENTHESIS', /\)/g, 0],
    ['INT', /\d+/g, 3],
    ['PLUS', /\+/g, 2],
    ['MINUS', /-/g, 2],
    ['MULTIPLY', /\*/g, 1],
    ['DIVIDE', /\//g, 1],
];
const process = (_code, _tokenList, x) => {
    if (x.name === 'PRINT') {
        return `console.log`;
    } else {
        return x.value;
    }
};
console.log('====================================');
console.log(
    compilerCore(
        `print("Hello")\nprint(10 + 20 * 30)`,
        tokenList,
        process,
    ),
);
console.log('====================================');
