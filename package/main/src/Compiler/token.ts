const compilerToken = (
    code: string,
    tokenList: [string, RegExp, number | null][],
    UncategorizedTokenList: [string, RegExp, number | null][],
) => {
    const tokenListMax = Math.max(...tokenList.map((n) => n[2] ?? 0));
    let list: {
        name: string;
        value: string;
        count: number;
        index: number;
    }[][] = [];
    const tokenListName = tokenList.map((n) => n[0]);
    let outCode: [string[], string[]] = [
        code.split('\n'),
        code.split('\n'),
    ];
    console.log('====================================');
    console.log(outCode);
    console.log('====================================');
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
        code.split('\n')
            .filter((n) => n !== '')
            .forEach((_, index) => {
                count = {};
                outCode[1][index] = outCode[1][index].replace(
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
                                index: outCode[1][index].indexOf(
                                    match,
                                ),
                            });
                        } catch {
                            list[index] = [
                                {
                                    name: token[0],
                                    value: match,
                                    count: 0,
                                    index: outCode[1][index].indexOf(
                                        match,
                                    ),
                                },
                            ];
                        }
                        outCode[1][index] = outCode[1][index].replace(
                            match,
                            ` ${token[0]} `,
                        );
                        return ` ${token[0]} `;
                    },
                );
            });
    }
    outCode[1].forEach((i, index) => {
        for (const token of i
            .split(' ')
            .filter((n) => n !== '')
            .filter((n) => tokenListName.indexOf(n) === -1)) {
            UncategorizedTokenList.forEach((n) => {
                token.replace(n[1], (match) => {
                    try {
                        count[n[0]].value += 1;
                    } catch {
                        count[n[0]] = {
                            value: 0,
                        };
                    }
                    try {
                        list[index].push({
                            name: n[0],
                            value: match,
                            count: count[n[0]].value,
                            index: outCode[0][index].indexOf(match),
                        });
                    } catch {
                        list[index] = [
                            {
                                name: n[0],
                                value: match,
                                count: 0,
                                index: outCode[0][index].indexOf(
                                    match,
                                ),
                            },
                        ];
                    }
                    outCode[0][index] = outCode[0][index].replace(
                        match,
                        ` ${n[0]} `,
                    );
                    outCode[1][index] = outCode[1][index].replace(
                        token,
                        ` ${n[0]} `,
                    );

                    return ` ${n[0]} `;
                });
            });
        }
    });
    return code.split('\n').map((n, index) => {
        return {
            code: n,
            token: list[index].sort((a, b) => a.index - b.index),
            outCode: outCode[1][index],
        };
    });
};
export default compilerToken;
