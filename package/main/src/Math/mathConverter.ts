import { mathSeparator } from "./mathSeparator";

/**
 * nの2乗を展開します。
 *
 * 1250*1250 => 1500*1000+400*100+200*100+50*50
 *
 * @param {string} equation - 変換する数学的な式
 * @returns {string} 変換後の式
 */
export const mathConverter = (equation: string): string => {
  let convertedEquation = equation;

  while (true) {
    if (convertedEquation.includes("^") || convertedEquation.includes("*")) {
      // 乗算と累乗の処理
      const extractedData: [RegExpMatchArray | null, string[]] = [
        convertedEquation.match(/\d+\.?(\d+)?(\*|\^)\d+\.?(\d+)?/),
        [""],
      ];

      if (extractedData[0]) {
        extractedData[1] = extractedData[0][0]
          .split(/(\d+\.\d+)|(\d+)/g)
          .filter((number_) => {
            return number_ !== undefined && number_ !== "";
          });

        if (
          extractedData[1][0] === extractedData[1][2] ||
          (extractedData[1][2] && extractedData[1][1] === "^")
        ) {
          const [primary, remainder] = mathSeparator(extractedData[1][0]);

          if (primary) {
            convertedEquation = `${
              Number(extractedData[1][0]) + remainder
            }*${primary}+`;

            if (remainder <= 100) {
              convertedEquation += `${remainder}*${remainder}`;
            } else {
              convertedEquation += mathConverter(`${remainder}*${remainder}`);
              return convertedEquation;
            }
            return convertedEquation;
          }
        }
      }
    }
    return convertedEquation;
  }
};
