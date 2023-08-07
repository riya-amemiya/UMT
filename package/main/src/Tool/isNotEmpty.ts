/**
 * objectが空かどうかを確認する
 * @param obj
 * @returns {boolean} true: 空でない, false: 空
 */

export const isNotEmpty = (obj: object): boolean => {
  // objectが空かどうかを確認して
  return Object.keys(obj).length > 0;
  // trueかfalseを返す
};
