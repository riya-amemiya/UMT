export const rangeAdvance = (
  start: number,
  end?: number,
  conditionalExpression?: (num: number) => boolean,
) => {
  const arr = [];
  if (!end) {
    for (let i = 0; i <= start; i++) {
      if (conditionalExpression) {
        if (conditionalExpression(i)) {
          arr.push(i);
        }
      }
    }
    return arr;
  }
  for (let i = start; i <= end; i++) {
    if (conditionalExpression) {
      if (conditionalExpression(i)) {
        arr.push(i);
      }
    }
  }
  return arr;
};
