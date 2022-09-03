import dayOfWeek from '../../Tool/dayOfWeek';
import {
    dayType,
    MonthsWith31Days,
    MonthsWihout31Days,
} from '../../types/monType';

const dayOfWeekSimple = <
    T extends MonthsWith31Days | MonthsWihout31Days,
>(
    props?:
        | { yer?: number; mon?: number; day?: number }
        | `${number}-${T}-${dayType<T>}`
        | `${number}:${T}:${dayType<T>}`
        | `${number}/${T}/${dayType<T>}`
        | Date,
    timeDifference: number = 9,
) => {
    if (typeof props === 'string') {
        if (props.includes(':')) {
            const [yer, mon, day] = props.split(':').map(Number);
            return dayOfWeek({ yer, mon, day }, timeDifference);
        } else if (props.includes('/')) {
            const [yer, mon, day] = props.split('/').map(Number);
            return dayOfWeek({ yer, mon, day }, timeDifference);
        } else {
            const [yer, mon, day] = props.split('-').map(Number);
            return dayOfWeek({ yer, mon, day }, timeDifference);
        }
    } else if (props instanceof Date) {
        return dayOfWeek(
            {
                yer: props.getFullYear(),
                mon: props.getMonth(),
                day: props.getDate(),
            },
            timeDifference,
        );
    } else {
        return dayOfWeek(props, timeDifference);
    }
};
export default dayOfWeekSimple;
