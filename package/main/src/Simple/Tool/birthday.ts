import birthday from '../../Tool/birthday';
import { dayType, monType } from '../../types/int';

const birthdaySimple = (
    birthdays:
        | `${number}-${monType}-${dayType}`
        | `${number}:${monType}:${dayType}`
        | `${number}/${monType}/${dayType}`
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
birthdaySimple('2020/10/01');
export default birthdaySimple;
