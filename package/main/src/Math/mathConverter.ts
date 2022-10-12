import { mathSeparator } from './mathSeparator';

const mathConverter = (x: string): string => {
    while (true) {
        if (x.indexOf('^') != -1 || x.indexOf('*') != -1) {
            //掛け算と割り算の処理
            const y: [RegExpMatchArray | null, string[]] = [
                x.match(/\d+\.?(\d+)?(\*|\^)\d+\.?(\d+)?/),
                [''],
            ];
            if (y[0]) {
                y[1] = y[0][0]
                    .split(/(\d+\.\d+)|(\d+)/g)
                    .filter((n) => {
                        return typeof n != 'undefined' && n != '';
                    });
                if (
                    y[1][0] == y[1][2] ||
                    (y[1][2] && y[1][1] == '^')
                ) {
                    let [n, m] = mathSeparator(y[1][0]);

                    if (n) {
                        x = `${Number(y[1][0]) + m}*${n}+`;
                        if (m <= 100) {
                            x += `${m}*${m}`;
                        } else {
                            return (x += mathConverter(`${m}*${m}`));
                        }
                        return x;
                    }
                }
                return x;
            } else {
                return x;
            }
        } else {
            return x;
        }
    }
};
export { mathConverter };
