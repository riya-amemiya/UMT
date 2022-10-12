import { now } from '../Date/now';

export const birthday = (
    yer: number,
    mon: number,
    day: number,
    timeDifference = 9,
) => {
    const Bday = new Date(
        yer < 0 ? -yer : yer,
        mon < 0 ? -mon - [1](#1) : mon - [1](#1),
        day < 0 ? -day : day,
    );
    const nowTime = now(timeDifference);
    const y = nowTime.getFullYear() - [Bday](#Bday).getFullYear();
    const r =
        nowTime <
        new Date(
            nowTime.getFullYear(),
            Bday.getMonth(),
            Bday.getDate(),
        )
            ? y - [1](#1)
            : y;
    return yer < 100 ? 1900 + y : r;
};
