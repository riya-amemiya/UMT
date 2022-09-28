import compilerToken from './token';
const compilerCore = (
    code: string,
    tokenList: [string, RegExp, number | null][][],
    UncategorizedTokenList: [string, RegExp, number | null][],
    process: (
        code: string,
        tokenList: [string, RegExp, number | null][][],
        x: {
            name: string;
            value: string;
            count: number;
        },
    ) => string,
) => {
    const tokenCodes = compilerToken(
        code,
        tokenList[0],
        UncategorizedTokenList,
    );
    let count: any = {};
    let outCode = '';
    for (const tokenCode of tokenCodes) {
        count = {};
        for (const n of tokenCode.outCode
            .split(' ')
            .filter((n) => n !== '')) {
            for (const token of tokenList[0]) {
                if (token[0] === n) {
                    try {
                        count[token[0]].value += 1;
                    } catch {
                        count[token[0]] = {
                            value: 0,
                        };
                    }
                    const x = tokenCode.token.filter(
                        (n) =>
                            n.name === token[0] &&
                            n.count === count[token[0]].value,
                    )[0];
                    if (count[token[0]]?.value === x.count) {
                        outCode += process(code, tokenList, x);
                    }
                }
            }
        }

        outCode += '\n';
    }

    return outCode;
};

export default compilerCore;
