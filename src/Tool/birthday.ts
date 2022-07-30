import now from '../Date/now';

const birthday = (yer: number, mon: number, day: number) => {
    const birthday = new Date(
        yer < 0 ? -yer : yer,
        mon < 0 ? -mon - 1 : mon - 1,
        day < 0 ? -day : day,
    );
    const nowTime = now();
    const y = nowTime.getFullYear() - birthday.getFullYear();
    const r =
        nowTime <
        new Date(
            nowTime.getFullYear(),
            birthday.getMonth(),
            birthday.getDate(),
        )
            ? y - 1
            : y;
    return yer < 100 ? 1900 + y : r;
};
export default birthday;
