import { isNumber } from '../isNumber';
import { multiplication } from '../multiplication';

export const exchange = <T extends object>(n: string, props?: T) => {
    if (props) {
        for (const i in props) {
            if (n[0] == i) {
                if (isNumber(props[i])) {
                    return String(
                        multiplication(
                            Number(n.slice(1)),
                            Number(props[i]),
                        ),
                    );
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
