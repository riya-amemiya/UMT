const compilerToken = (
    code: string,
    tokenList: [string, RegExp, number | null][],
) => {
    const tokenListMax = Math.max(...tokenList.map((n) => n[2] ?? 0));
    let list: { name: string; value: string }[][] = [];
    let outCode: string[] = code.split('\n');
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
        code.split('\n').forEach((_, index) => {
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
export default compilerToken;
