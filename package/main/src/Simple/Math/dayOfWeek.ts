import dayOfWeek from '../../Tool/dayOfWeek';

const dayOfWeekSimple = (
    props?:
        | {
              yer?: number;
              mon?: number;
              day?: number;
          }
        | string
        | Date,
) => {
    if (typeof props === 'string') {
        const [yer, mon, day] = props.split('-').map(Number);
        return dayOfWeek({ yer, mon, day });
    } else if (props instanceof Date) {
        return dayOfWeek({
            yer: props.getFullYear(),
            mon: props.getMonth(),
            day: props.getDate(),
        });
    } else {
        return dayOfWeek(props);
    }
};
export default dayOfWeekSimple;
