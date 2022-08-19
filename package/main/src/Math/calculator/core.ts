import addition from '../addition';
import division from '../division';
import multiplication from '../multiplication';
import subtract from '../subtract';
import exchange from './exchange';
/**
 * 電卓
 * ()や符号に対応
 * xなどの文字は未対応
 * @param  {string} x 計算式
 */
const calculatorCore = <T extends object>(
    x: string,
    ex?: T,
): string => {
    //符号反転
    x = x.replace(/--/g, '+');
    x = x.replace(/\+\+/g, '+');
    x = x.replace(/\+-/g, '+0-');
    x = x.replace(/\-\+/g, '+0-');
    //円計算
    if (ex) {
        for (const i in ex) {
            if (x.indexOf(i) != -1) {
                const $ = x.match(new RegExp(`\\${i}` + '([0-9]+)'));
                if ($) {
                    return calculatorCore(
                        x.replace($[0], exchange($[0], ex)),
                        ex,
                    );
                }
            }
        }
    }
    //括弧の処理
    if (x.indexOf('(') != -1 || x.indexOf(')') != -1) {
        //括弧の中身をぬく
        const y = x.match(
            /\(\d+\.?(\d+)?(\*|\/|\+|\-)\d+\.?(\d+)?\)/,
        );
        if (y) {
            //括弧の中身を計算
            return calculatorCore(
                x.replace(
                    y[0],
                    calculatorCore(y[0].replace(/\(|\)/g, '')),
                ),
                ex,
            );
        } else {
            return calculatorCore(
                x.replace(
                    `(${x.slice(
                        x.indexOf('(') + 1,
                        x.indexOf(')'),
                    )})`,
                    calculatorCore(
                        x.slice(x.indexOf('(') + 1, x.indexOf(')')),
                    ),
                ),
                ex,
            );
        }
    } else if (
        x.indexOf('^') != -1 ||
        x.indexOf('*') != -1 ||
        x.indexOf('/') != -1
    ) {
        //掛け算と割り算の処理
        const y: [RegExpMatchArray | null, string[]] = [
            x.match(/\d+\.?(\d+)?(\*|\/|\^)\d+\.?(\d+)?/),
            [''],
        ];
        if (y[0]) {
            y[1] = y[0][0].split(/(\d+\.\d+)|(\d+)/g).filter((n) => {
                return typeof n != 'undefined' && n != '';
            });
            return calculatorCore(
                x.replace(
                    y[0][0],
                    `${
                        y[1][1] == '^'
                            ? Math.pow(+y[1][0], +y[1][2])
                            : y[1][1] == '*'
                            ? multiplication(+y[1][0], +y[1][2])
                            : y[1][1] == '/'
                            ? division(+y[1][0], +y[1][2])[0]
                            : '0'
                    }`,
                ),
                ex,
            );
        }
        return x;
    } else if (x.indexOf('+') != -1 || x.indexOf('-') != -1) {
        //加算と減算の処理
        let y: [RegExpMatchArray | null, string[]] = [
            x.match(/\d+\.?(\d+)?(\+|\-)\d+\.?(\d+)?/),
            [''],
        ];

        if (y[0]) {
            y[1] = y[0][0].split(/(\d+\.\d+)|(\d+)/g).filter((n) => {
                return typeof n != 'undefined' && n !== '';
            });
            return calculatorCore(
                x.replace(
                    y[0][0],
                    `${
                        y[1][1] == '+'
                            ? addition(
                                  Number(y[1][0]),
                                  Number(y[1][2]),
                              )
                            : y[1][1] == '-'
                            ? subtract(
                                  Number(y[1][0]),
                                  Number(y[1][2]),
                              )
                            : '0'
                    }`,
                ),
                ex,
            );
        }
        return x;
    } else {
        return x;
    }
};
export default calculatorCore;
