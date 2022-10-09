const roundOf = (num: number, precision: number) => {
    return (
        Math.round(num * Math.pow(10, precision)) /
        Math.pow(10, precision)
    );
};

export default roundOf;
