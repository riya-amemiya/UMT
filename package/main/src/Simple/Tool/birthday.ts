import birthday from '../../Tool/birthday';

const birthdaySimple = (
    birthdays:
        | string
        | Date
        | { yer: number; mon: number; day: number },
) => {
    if (typeof birthdays === 'string') {
        const [yer, mon, day] = birthdays.split('-').map(Number);
        return birthday(yer, mon, day);
    } else if (birthdays instanceof Date) {
        return birthday(
            birthdays.getFullYear(),
            birthdays.getMonth(),
            birthdays.getDate(),
        );
    } else {
        return birthday(birthdays.yer, birthdays.mon, birthdays.day);
    }
};
export default birthdaySimple;
