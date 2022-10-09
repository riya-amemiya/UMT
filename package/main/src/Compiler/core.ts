import compilerToken from './token';
const compilerCore = (
    code: string,
    tokenList: [string, RegExp, number | null][],
    UncategorizedTokenList: [string, RegExp, number | null][],
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
    const tokenCodes = compilerToken(
        code,
        tokenList,
        UncategorizedTokenList,
    );
    console.log('====================================');
    console.log(tokenCodes[0].token);
    console.log('====================================');
    let outCode = '';
    for (const tokenCode of tokenCodes) {
        for (const n of tokenCode.token) {
            outCode += process(code, tokenList, n);
        }
        outCode += '\n';
    }

    return outCode;
};

export default compilerCore;
