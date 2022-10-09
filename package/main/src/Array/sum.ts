import addition from '../Math/addition';
const sum = (x: number[]) => {
    return x.reduce((a, b) => addition(a, b));
};

export default sum;
