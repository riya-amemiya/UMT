/**
 * objectが空かどうかを確認する
 * @param {object} object - 空かどうかを確認するobject
 * @returns {boolean} true: 空でない, false: 空
 * @example isNotEmpty({}); // false
 * isNotEmpty({ a: 1 }); // true
 */
export const isNotEmpty = (object: object): boolean => {
  // objectが空かどうかを確認する
  return Object.keys(object).length > 0;
};
