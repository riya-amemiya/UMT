'use strict';
const compilerToken = (code, tokenList) => {
    const tokenListMax = Math.max(
        ...tokenList.map((n) => {
            var _a;
            return (_a = n[2]) !== null && _a !== void 0 ? _a : 0;
        }),
    );
    let list = [];
    let outCode = code.split('\n');
    tokenList.forEach((token) => {
        if (token[2] === null) {
            return (token[2] = tokenListMax + 1);
        } else {
            return token;
        }
    });
    tokenList.sort((a, b) => {
        if (a[2] === null) {
            return 1;
        } else if (b[2] === null) {
            return -1;
        } else {
            return a[2] - b[2];
        }
    });
    tokenList.forEach((token) => {
        code.split('\n').forEach((n, index) => {
            outCode[index] = outCode[index].replace(
                new RegExp(token[1], 'g'),
                (match) => {
                    try {
                        list[index].push({
                            name: token[0],
                            value: match,
                        });
                    } catch {
                        list[index] = [
                            { name: token[0], value: match },
                        ];
                    }
                    return ` ${token[0]} `;
                },
            );
        });
    });
    return code.split('\n').map((n, index) => {
        return {
            code: n,
            token: list[index],
            outCode: outCode[index],
        };
    });
};

const compilerCore = (code, tokenList) => {
    const tokens = compilerToken(code, tokenList);
    tokens.forEach((token) => {
        token.outCode
            .split(' ')
            .filter((n) => n !== '')
            .forEach((n) => {
                console.log(n);
            });
    });
};

console.log('====================================');
console.log(
    compilerCore(`print("Hello")\nprint(10)`, [
        ['PRINT', /print/, null],
        ['LPAREN', /\(/, 1],
        ['RPAREN', /\)/, 0],
        ['STRING', /"[^"]*"/, null],
        ['INT', /\d+/, null],
    ]),
);
console.log('====================================');
