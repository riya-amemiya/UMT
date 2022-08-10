import multiplication from '../multiplication';

const exchange = ({ n, $ }: { n: string; $?: number }) => {
    if ($) {
        if (n[0] === '$') {
            return String(multiplication(Number(n.slice(1)), $));
        } else {
            return n;
        }
    } else {
        return n;
    }
};
export default exchange;
