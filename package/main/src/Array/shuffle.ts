/**
 * Randomly shuffles the elements of an array
 * @param array Array to shuffle
 * @returns New array with shuffled elements
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

  if (shuffledArray.length >= 2) {
    let isSame = true;
    for (let i = 0; i < array.length; i++) {
      if (shuffledArray[i] !== array[i]) {
        isSame = false;
        break;
      }
    }

    if (isSame) {
      [shuffledArray[0], shuffledArray[1]] = [shuffledArray[1], shuffledArray[0]];
    }
  }

  return shuffledArray;
};
