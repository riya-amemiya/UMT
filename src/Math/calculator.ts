import addition from './addition';
import division from './division';
import multiplication from './multiplication';
import subtract from './subtract';
/**
 * 電卓()や符号に対応
 * xなどの文字は未対応
 * @param  {string} x 計算式
 */

const calculator = (x: string, n: number = 0): string => {
    if (n === 0) {
        x = x.replace(/\s+/g, '');

        n++;
    }
    x = x.replace(/--/g, '+');
    x = x.replace(/\+\+/g, '+');
    x = x.replace(/\+-/g, '+0-');
    x = x.replace(/\-\+/g, '+0-');

    if (x.indexOf('(') != -1 || x.indexOf(')') != -1) {
        const y = x.match(
            /\(\d+\.?(\d+)?(\*|\/|\+|\-)\d+\.?(\d+)?\)/,
        );
        if (y) {
            return calculator(
                x.replace(
                    y[0],
                    calculator(y[0].replace(/\(|\)/g, ''), n),
                ),
                n,
            );
        } else {
            return calculator(
                x.replace(
                    `(${x.slice(
                        x.indexOf('(') + 1,
                        x.indexOf(')'),
                    )})`,
                    calculator(
                        x.slice(x.indexOf('(') + 1, x.indexOf(')')),
                        n,
                    ),
                ),
                n,
            );
        }
    } else if (x.indexOf('*') != -1 || x.indexOf('/') != -1) {
        const y: [RegExpMatchArray | null, string[]] = [
            x.match(/\d+\.?(\d+)?(\*|\/)\d+\.?(\d+)?/),
            [''],
        ];
        if (y[0]) {
            y[1] = y[0][0].split(/(\d+\.\d+)|(\d+)/g).filter((n) => {
                return typeof n != 'undefined' && n != '';
            });
            return calculator(
                x.replace(
                    y[0][0],
                    `${
                        y[1][1] == '*'
                            ? multiplication(
                                  Number(y[1][0]),
                                  Number(y[1][2]),
                              )
                            : y[1][1] == '/'
                            ? division(
                                  Number(y[1][0]),
                                  Number(y[1][2]),
                              )[0]
                            : '0'
                    }`,
                ),
                n,
            );
        }
        return x;
    } else if (x.indexOf('+') != -1 || x.indexOf('-') != -1) {
        let y: [RegExpMatchArray | null, string[]] = [
            x.match(/\d+\.?(\d+)?(\+|\-)\d+\.?(\d+)?/),
            [''],
        ];

        if (y[0]) {
            y[1] = y[0][0].split(/(\d+\.\d+)|(\d+)/g).filter((n) => {
                return typeof n != 'undefined' && n !== '';
            });
            return calculator(
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
                n,
            );
        }
        return x;
    } else {
        return x;
    }
};
export default calculator;
