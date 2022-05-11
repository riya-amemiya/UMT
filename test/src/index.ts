import euclideanAlgorithm from 'umt/build/euclideanAlgorithm';
const test = <X extends unknown[][]>(x: X) => {
    for (const i of x) {
        if (typeof i[0] != 'object' && !Array.isArray(i[0])) {
            if (!(i[0] === i[1])) {
                throw `Error x:${i[0]},y:${i[1]}`;
            }
        } else if (Array.isArray(i[0])) {
            for (let n = 0; n < i[0].length; n++) {
                if (
                    !(
                        JSON.stringify(i[n]) ===
                        JSON.stringify(
                            i[i[0].length < n + 1 ? n + 1 : 0],
                        )
                    )
                ) {
                    throw `Error out:${i[0]},in:${i[1]}`;
                }
            }
        } else if (typeof i[0] === 'boolean') {
            if (!i[0] && i[1]) {
                throw `Error out:${i[0]},in:${i[1]}`;
            }
        }
    }
};
test([[euclideanAlgorithm(910, 2190, 2121), 1]]);
