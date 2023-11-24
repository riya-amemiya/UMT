/**
 * 配列の要素をランダムにシャッフルする
 * @param array シャッフルする配列
 * @returns シャッフルされた配列
 * @example shuffle([1, 2, 3, 4, 5]); // [3, 5, 2, 4, 1]
 */
export const shuffle = <T>(array: T[]): T[] => {
  const shuffledArray = [...array];
  for (let index = shuffledArray.length - 1; index > 0; index--) {
    const index_ = Math.floor(Math.random() * (index + 1));
    [shuffledArray[index], shuffledArray[index_]] = [
      shuffledArray[index_],
      shuffledArray[index],
    ];
  }
  return shuffledArray;
};
