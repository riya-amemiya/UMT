import { dayOfWeek } from '../../Tool/dayOfWeek';
import {
    MonthsWihout31Days,
    MonthsWihout31DaysInt,
    MonthsWith31Days,
    MonthsWith31DaysInt,
    dayType,
    dayTypeInt,
} from '../../types/monType';

export const dayOfWeekSimple = <
    T extends MonthsWith31Days | MonthsWihout31Days,
>(
    props?:
        | { yer?: number; mon?: T; day?: dayType<T> }
        | `${number}-${T}-${dayType<T>}`
        | `${number}:${T}:${dayType<T>}`
        | `${number}/${T}/${dayType<T>}`
        | Date,
    timeDifference = 9,
) => {
    if (typeof props === 'string') {
        if (props.includes(':')) {
            const [yer, mon, day] = props
                .split(':')
                // rome-ignore lint/suspicious/noExplicitAny: <explanation>
                .map(Number) as any;
            return dayOfWeek({ yer, mon, day }, timeDifference);
        } else if (props.includes('/')) {
            const [yer, mon, day] = props
                .split('/')
                // rome-ignore lint/suspicious/noExplicitAny: <explanation>
                .map(Number) as any;
            return dayOfWeek({ yer, mon, day }, timeDifference);
        } else {
            const [yer, mon, day] = props
                .split('-')
                // rome-ignore lint/suspicious/noExplicitAny: <explanation>
                .map(Number) as any;
            return dayOfWeek({ yer, mon, day }, timeDifference);
        }
    } else if (props instanceof Date) {
        return dayOfWeek(
            {
                yer: props.getFullYear(),
                mon: props.getMonth() as
                    | MonthsWihout31DaysInt
                    | MonthsWith31DaysInt,
                day: props.getDate() as dayTypeInt<
                    MonthsWith31DaysInt | MonthsWihout31DaysInt
                >,
            },
            timeDifference,
        );
    } else {
        return dayOfWeek(
            props as {
                yer?: number;
                mon?: MonthsWihout31DaysInt | MonthsWith31DaysInt;
                day?: dayTypeInt<
                    MonthsWith31DaysInt | MonthsWihout31DaysInt
                >;
            },
            timeDifference,
        );
    }
};
