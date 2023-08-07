function range(end: number): number[];
function range(start: number, end: number): number[];
function range(start: number, end?: number) {
  const arr = [];
  if (!end) {
    for (let i = 0; i < start; i++) {
      arr.push(i);
    }
    return arr;
  }
  for (let i = start; i <= end; i++) {
    arr.push(i);
  }
  return arr;
}
export { range };
