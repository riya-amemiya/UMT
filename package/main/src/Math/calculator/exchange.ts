import { isNumber } from '../isNumber';
import { multiplication } from '../multiplication';
// The function accepts two parameters, the first is n, a string, and the second is props, an object.
// props is an optional parameter.
export const exchange = <T extends object>(n: string, props?: T) => {
    // If the props parameter exists, then the following code will be executed.
    if (props) {
        // Loop through the object props.
        for (const i in props) {
            // If the first character of the string n is equal to the key of the current object item,
            // then the following code will be executed.
            if (n[0] == i) {
                // If the value of the current object item is a number, then the following code will be executed.
                if (isNumber(props[i])) {
                    // Return the result of the multiplication of the number n after the first character and the value of the current object item.
                    return String(
                        multiplication(
                            Number(n.slice(1)),
                            Number(props[i]),
                        ),
                    );
                }
            } else {
                // If the first character of the string n is not equal to the key of the current object item,
                // then return the original string n.
                return n;
            }
        }
        // If the string n does not have a corresponding key in the object props, then return the original string n.
        return n;
    } else {
        // If the props parameter does not exist, then return the original string n.
        return n;
    }
};
