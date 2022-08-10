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
interface Props {
    $?: number;
}
const calculatorCore = (x: string, ex?: Props): string => {
    x = x.replace(/--/g, '+');
    x = x.replace(/\+\+/g, '+');
    x = x.replace(/\+-/g, '+0-');
    x = x.replace(/\-\+/g, '+0-');
    if (x.indexOf('$') != -1) {
        if (ex) {
            const $ = x.match(/(\$[0-9]+)/);
            if ($) {
                return calculatorCore(
                    x.replace($[0], exchange({ n: $[0], $: ex.$ })),
                );
            }
        }
    }
    if (x.indexOf('(') != -1 || x.indexOf(')') != -1) {
        const y = x.match(
            /\(\d+\.?(\d+)?(\*|\/|\+|\-)\d+\.?(\d+)?\)/,
        );
        if (y) {
            return calculatorCore(
                x.replace(
                    y[0],
                    calculatorCore(y[0].replace(/\(|\)/g, '')),
                ),
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
            return calculatorCore(
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
            );
        }
        return x;
    } else {
        return x;
    }
};
export default calculatorCore;
