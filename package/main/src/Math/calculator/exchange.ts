import { isNumber } from '../isNumber';
import { multiplication } from '../multiplication';
// The function accepts two parameters, the first is n, a string, and the second is props, an object.
// props is an optional parameter.
export const exchange = <T extends object>(n: string, props?: T) => {
    if (props) {
        for (const i in props) {
            if (n.indexOf(i) != -1) {
                if (isNumber(props[i])) {
                    let x = multiplication(
                        Number(n.slice(i.length)),
                        Number(props[i]),
                    );
                    if (isNaN(x)) {
                        return n;
                    } else {
                        return String(
                            multiplication(
                                Number(n.slice(i.length)),
                                Number(props[i]),
                            ),
                        );
                    }
                }
            } else {
                return n;
            }
        }
        return n;
    } else {
        return n;
    }
};
