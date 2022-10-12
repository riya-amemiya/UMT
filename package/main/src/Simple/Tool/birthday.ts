import { birthday } from '../../Tool/birthday';
import {
    dayType,
    MonthsWihout31Days,
    MonthsWith31Days,
} from '../../types/monType';
export const birthdaySimple = <
    T extends MonthsWith31Days | MonthsWihout31Days,
>(
    birthdays:
        | `${number}-${T}-${dayType<T>}`
        | `${number}:${T}:${dayType<T>}`
        | `${number}/${T}/${dayType<T>}`
        | Date
        | { yer: number; mon: number; day: number },
    timeDifference: number = 9,
) => {
    if (typeof birthdays === 'string') {
        if (birthdays.includes(':')) {
            const [yer, mon, day] = birthdays.split(':').map(Number);
            return birthday(yer, mon, day, timeDifference);
        } else if (birthdays.includes('/')) {
            const [yer, mon, day] = birthdays.split('/').map(Number);
            return birthday(yer, mon, day, timeDifference);
        } else {
            const [yer, mon, day] = birthdays.split('-').map(Number);
            return birthday(yer, mon, day, timeDifference);
        }
    } else if (birthdays instanceof Date) {
        return birthday(
            birthdays.getFullYear(),
            birthdays.getMonth(),
            birthdays.getDate(),
            timeDifference,
        );
    } else {
        return birthday(
            birthdays.yer,
            birthdays.mon,
            birthdays.day,
            timeDifference,
        );
    }
};
