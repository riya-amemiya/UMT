import compilerToken from './token';
const compilerCore = (
    code: string,
    tokenList: [string, RegExp, number | null][],
    process: (
        code: string,
        tokenList: [string, RegExp, number | null][],
        x: {
            name: string;
            value: string;
            count: number;
        },
    ) => string,
) => {
    const tokenCodes = compilerToken(code, tokenList);
    let count: any = {};
    let outCode = '';
    tokenCodes.forEach((tokenCode) => {
        count = {};
        tokenCode.outCode
            .split(' ')
            .filter((n) => n !== '')
            .forEach((n) => {
                tokenList.forEach((token) => {
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
                });
            });
        outCode += '\n';
    });
    return outCode;
};

export default compilerCore;
