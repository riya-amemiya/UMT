import { mathSeparator } from "./mathSeparator";

/**
 * @param {string} x
 * @returns string
 */

export const mathConverter = (x: string): string => {
<<<<<<< HEAD
  let returnValue = x;
  while (true) {
    if (returnValue.indexOf("^") !== -1 || returnValue.indexOf("*") !== -1) {
      //掛け算と割り算の処理
      const y: [RegExpMatchArray | null, string[]] = [
        returnValue.match(/\d+\.?(\d+)?(\*|\^)\d+\.?(\d+)?/),
        [""],
      ];
      if (y[0]) {
        y[1] = y[0][0].split(/(\d+\.\d+)|(\d+)/g).filter((n) => {
          return typeof n !== "undefined" && n !== "";
        });
        if (y[1][0] === y[1][2] || (y[1][2] && y[1][1] === "^")) {
          const [n, m] = mathSeparator(y[1][0]);

          if (n) {
            returnValue = `${Number(y[1][0]) + m}*${n}+`;
            if (m <= 100) {
              returnValue += `${m}*${m}`;
            } else {
              returnValue += mathConverter(`${m}*${m}`);
              return returnValue;
            }
            return returnValue;
          }
        }
        return returnValue;
      } else {
        return returnValue;
      }
    } else {
      return returnValue;
    }
  }
=======
	let returnValue = x;
	let flag = true;
	while (flag) {
		if (returnValue.indexOf("^") !== -1 || returnValue.indexOf("*") !== -1) {
			//掛け算と割り算の処理
			const y: [RegExpMatchArray | null, string[]] = [
				returnValue.match(/\d+\.?(\d+)?(\*|\^)\d+\.?(\d+)?/),
				[""],
			];
			if (y[0]) {
				y[1] = y[0][0].split(/(\d+\.\d+)|(\d+)/g).filter((n) => {
					return typeof n !== "undefined" && n !== "";
				});
				if (y[1][0] === y[1][2] || (y[1][2] && y[1][1] === "^")) {
					const [n, m] = mathSeparator(y[1][0]);

					if (n) {
						returnValue = `${Number(y[1][0]) + m}*${n}+`;
						if (m <= 100) {
							returnValue += `${m}*${m}`;
						} else {
							returnValue += mathConverter(`${m}*${m}`);
							flag = false;
							break;
						}
						flag = false;
						break;
					}
				}
				flag = false;
				break;
			} else {
				flag = false;
				break;
			}
		} else {
			flag = false;
			break;
		}
	}
	return returnValue;
>>>>>>> 58a3e53 (修正)
};
