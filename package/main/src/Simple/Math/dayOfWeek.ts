import dayOfWeek from '../../Tool/dayOfWeek';

const dayOfWeekSimple = (
    props?:
        | { yer?: number; mon?: number; day?: number }
        | string
        | Date,
    timeDifference: number = 9,
) => {
    if (typeof props === 'string') {
        const [yer, mon, day] = props.split('-').map(Number);
        return dayOfWeek({ yer, mon, day }, timeDifference);
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
