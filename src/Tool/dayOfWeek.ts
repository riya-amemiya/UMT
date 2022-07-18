const dayOfWeek = ({
    yer,
    mon,
    day,
}: {
    yer: number;
    mon: number;
    day: number;
}) => new Date(yer, mon - 1, day).getDay();
export default dayOfWeek;
