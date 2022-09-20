import compilerToken from './token';
const compilerCore = (
    code: string,
    tokenList: [string, RegExp, number | null][],
) => {
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
export default compilerCore;
