const compilerToken = (
    code: string,
    tokenList: [string, RegExp, number | null][],
    UncategorizedTokenList: [string, RegExp, number | null][],
) => {
    const tokenListMax = Math.max(...tokenList.map((n) => n[2] ?? 0));
    let list: { name: string; value: string; count: number }[][] =
        tokenList.map((n) => {
            return [{ name: n[0], value: '', count: 0 }];
        });
    const tokenListName = tokenList.map((n) => n[0]);
    let outCode: string[] = code.split('\n');
    let count: any = {};
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
    for (const token of tokenList) {
        for (const index in code.split('\n')) {
            count = {};
            outCode[index] = outCode[index].replace(
                new RegExp(token[1], 'g'),
                (match) => {
                    try {
                        count[token[0]].value += 1;
                    } catch {
                        count[token[0]] = {
                            value: 0,
                        };
                    }
                    try {
                        list[index].push({
                            name: token[0],
                            value: match,
                            count: count[token[0]].value,
                        });
                    } catch {
                        list[index] = [
                            {
                                name: token[0],
                                value: match,
                                count: 0,
                            },
                        ];
                    }
                    return ` ${token[0]} `;
                },
            );
        }
    }
    outCode.forEach((i, index) => {
        for (const token of i
            .split(' ')
            .filter((n) => n !== '')
            .filter((n) => tokenListName.indexOf(n) === -1)) {
            for (const n of UncategorizedTokenList) {
                token.replace(n[1], (match) => {
                    try {
                        count[n[0]].value += 1;
                    } catch {
                        count[n[0]] = {
                            value: 0,
                        };
                    }
                    try {
                        list[outCode.indexOf(i)].push({
                            name: n[0],
                            value: match,
                            count: count[n[0]].value,
                        });
                    } catch {
                        list[outCode.indexOf(i)] = [
                            {
                                name: n[0],
                                value: match,
                                count: 0,
                            },
                        ];
                    }
                    outCode[index] = outCode[index].replace(
                        token,
                        ` ${n[0]} `,
                    );
                    return ` ${n[0]} `;
                });
            }
        }
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
