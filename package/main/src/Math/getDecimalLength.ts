/**
Gets the number of decimal places in a number
@param  {number} value - Number to check
@returns {number} Number of decimal places (0 for integers)
@example
getDecimalLength(1.23); // 2
getDecimalLength(100); // 0
getDecimalLength(1.0); // 0
*/
export const getDecimalLength = (value: number) => {
  const string_ = value.toString();
  const index = string_.indexOf(".");
  return index === -1 ? 0 : string_.length - index - 1;
};
