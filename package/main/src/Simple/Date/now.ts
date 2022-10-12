import { now } from '../../Date';
import { isNumber } from '../../Math';

const nowSimple = (timeDifference: number | string = 9) => {
    if (typeof timeDifference === 'number') {
        return now(timeDifference);
    } else if (isNumber(timeDifference)) {
        return now(Number(timeDifference));
    } else {
        return now();
    }
};
export { nowSimple };
