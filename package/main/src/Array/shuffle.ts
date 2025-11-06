/**
 * Randomly shuffles the elements of an array
 * @param array Array to shuffle
 * @returns New array with shuffled elements
 * @example shuffle([1, 2, 3, 4, 5]); // [3, 5, 2, 4, 1]
 */
export const shuffle = <T>(array: T[]): T[] => {
  const shuffledArray = [...array];

  for (let index = shuffledArray.length - 1; index > 0; index--) {
    const randomIndex = Math.floor(Math.random() * (index + 1));
    const index_ =
      index === shuffledArray.length - 1 &&
      shuffledArray.length >= 2 &&
      randomIndex === index
        ? index - 1
        : randomIndex;

    [shuffledArray[index], shuffledArray[index_]] = [
      shuffledArray[index_],
      shuffledArray[index],
    ];
  }

  return shuffledArray;
};
