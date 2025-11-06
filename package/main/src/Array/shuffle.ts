/**
 * Randomly shuffles the elements of an array
 * @param array Array to shuffle
 * @returns New array with shuffled elements
 * @example shuffle([1, 2, 3, 4, 5]); // [3, 5, 2, 4, 1]
 */
export const shuffle = <T>(array: T[]): T[] => {
  const shuffledArray = [...array];

  for (let index = shuffledArray.length - 1; index > 0; index--) {
    let index_ = Math.floor(Math.random() * (index + 1));

    // For arrays with 2+ elements, ensure the first swap moves the last element
    if (
      index === shuffledArray.length - 1 &&
      shuffledArray.length >= 2 &&
      index_ === index
    ) {
      index_ = index - 1;
    }

    [shuffledArray[index], shuffledArray[index_]] = [
      shuffledArray[index_],
      shuffledArray[index],
    ];
  }

  return shuffledArray;
};
