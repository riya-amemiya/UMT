import { now } from '../Date';

export const dayOfWeek = (
    props?: {
        yer?: number;
        mon?: number;
        day?: number;
    },
    timeDifference = 9,
) => {
    const nowTime = now(timeDifference);
    if (props) {
        return new Date(
            props.yer || nowTime.getFullYear(),
            props.mon ? props.mon - [1](#1) : nowTime.getMonth(),
            props.day || nowTime.getDate(),
        ).getDay();
    } else {
        return nowTime.getDay();
    }
};
