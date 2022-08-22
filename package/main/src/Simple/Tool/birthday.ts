import birthday from '../../Tool/birthday';

const birthdaySimple = (
    birthdays:
        | string
        | Date
        | { yer: number; mon: number; day: number },
    timeDifference: number = 9,
) => {
    if (typeof birthdays === 'string') {
        const [yer, mon, day] = birthdays.split('-').map(Number);
        return birthday(yer, mon, day, timeDifference);
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
export default birthdaySimple;
