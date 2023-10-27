import { division } from "../division";
import { gcd } from "../gcd";
import { calculatorCore } from "./core";
/**
 * 方程式計算
 * @param  {string} x
 */
export const literalExpression = (x: string) => {
  const cache: [string[], string] = [[], ""];
  for (const index of x.split("=")) {
    if (/[A-Za-z]+/.test(index) === false) {
      cache[1] = index;
    } else {
      cache[0] = index
        .split(/(\d+[A-Za-z]+)|([^A-Za-z]+)/)
        .filter((n) => n !== "" && n !== undefined);
    }
  }
  if (cache[0][1]) {
    cache[0][1] = calculatorCore(cache[0][1]);
    if (cache[0][1].includes("+")) {
      cache[0][1] = cache[0][1].replaceAll("+", "plus");
    }
    if (cache[0][1].includes("-")) {
      cache[0][1] = cache[0][1].replaceAll("-", "minus");
    }
    if (cache[0][1].indexOf("plus") !== 1) {
      cache[0][1] = cache[0][1].replaceAll("plus", "-");
    }
    if (cache[0][1].indexOf("minus") !== 1) {
      cache[0][1] = cache[0][1].replaceAll("minus", "+");
    }
  }
  cache[1] = cache[0][1]
    ? calculatorCore(`${cache[1]}${cache[0][1]}`)
    : calculatorCore(cache[1]);
  cache[0] = cache[0][0]
    .split(/(\d+)|([A-Za-z]+)/)
    .filter((n) => n !== "" && n !== undefined);
  if (Number.isNaN(Number(cache[0][0]))) {
    return cache[1];
  }
  const cacheGcd = gcd(Number(cache[0][0]), Number(cache[1]));
  if (cacheGcd !== 1) {
    return `${division(Number(cache[1]), cacheGcd)}/${division(
      Number(cache[0][0]),
      cacheGcd,
    )}`;
  }
  return cache[0][0] === "1" ? `${cache[1]}/${cache[0][0]}` : cache[1];
};
