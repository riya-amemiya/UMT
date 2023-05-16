import { addition } from "@/Math/addition";
import { division } from "@/Math/division";
import { multiplication } from "@/Math/multiplication";
import { subtract } from "@/Math/subtract";
import { exchange } from "./exchange";
/**
 * 電卓
 * ()や符号に対応
 * xなどの文字は未対応
 * @param  {string} x 計算式
 */
export const calculatorCore = <T extends object>(x: string, ex?: T): string => {
	let copyX = x;
	//符号反転
	copyX = copyX.replace(/--/g, "+");
	copyX = copyX.replace(/\+\+/g, "+");
	copyX = copyX.replace(/\+-/g, "+0-");
	copyX = copyX.replace(/\-\+/g, "+0-");
	let flag = true;
	//円計算
	while (flag) {
		if (ex) {
			for (const i in ex) {
				if (copyX.indexOf(i) !== -1) {
					const $ = copyX.match(new RegExp(`\\${i}([0-9]+)`));
					if ($) {
						copyX = copyX.replace($[0], exchange($[0], ex));
					}
				}
			}
		}
		//括弧の処理
		if (copyX.indexOf("(") !== -1 || copyX.indexOf(")") !== -1) {
			//括弧の中身をぬく
			const y = copyX.match(/\(\d+\.?(\d+)?(\*|\/|\+|\-)\d+\.?(\d+)?\)/);
			if (y) {
				//括弧の中身を計算
				copyX = copyX.replace(y[0], calculatorCore(y[0].replace(/\(|\)/g, "")));
			} else {
				copyX = copyX.replace(
					`(${copyX.slice(copyX.indexOf("(") + 1, copyX.indexOf(")"))})`,
					calculatorCore(
						copyX.slice(copyX.indexOf("(") + 1, copyX.indexOf(")")),
					),
				);
			}
		} else if (
			copyX.indexOf("^") !== -1 ||
			copyX.indexOf("*") !== -1 ||
			copyX.indexOf("/") !== -1
		) {
			//掛け算と割り算の処理
			const y: [RegExpMatchArray | null, string[]] = [
				copyX.match(/\d+\.?(\d+)?(\*|\/|\^)\d+\.?(\d+)?/),
				[""],
			];
			if (y[0]) {
				y[1] = y[0][0].split(/(\d+\.\d+)|(\d+)/g).filter((n) => {
					return typeof n !== "undefined" && n !== "";
				});
				copyX = copyX.replace(
					y[0][0],
					`${
						y[1][1] === "^"
							? (+y[1][0]) ** +y[1][2]
							: y[1][1] === "*"
							? multiplication(+y[1][0], +y[1][2])
							: y[1][1] === "/"
							? division(+y[1][0], +y[1][2])
							: "0"
					}`,
				);
			}
		} else if (copyX.indexOf("+") !== -1 || copyX.indexOf("-") !== -1) {
			//加算と減算の処理
			const y: [RegExpMatchArray | null, string[]] = [
				copyX.match(/\d+\.?(\d+)?(\+|\-)\d+\.?(\d+)?/),
				[""],
			];
			if (y[0]) {
				y[1] = y[0][0].split(/(\d+\.\d+)|(\d+)/g).filter((n) => {
					return typeof n !== "undefined" && n !== "";
				});
				copyX = copyX.replace(
					y[0][0],
					`${
						y[1][1] === "+"
							? addition(Number(y[1][0]), Number(y[1][2]))
							: y[1][1] === "-"
							? subtract(Number(y[1][0]), Number(y[1][2]))
							: "0"
					}`,
				);
			}
		} else {
			flag = false;
			break;
		}
	}
	return copyX;
};
