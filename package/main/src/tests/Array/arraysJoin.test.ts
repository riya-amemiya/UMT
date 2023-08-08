import { arraysJoin } from "../../../module/Array/arraysJoin";
test("{arraysJoin}", () => {
  expect(arraysJoin([1, 2, 3], [4, 5, 6])).toEqual(
    expect.arrayContaining([1, 2, 3, 4, 5, 6]),
  );
  expect(arraysJoin([1, 2, 3], [4, 5, 6], [7, 8, 9])).toEqual(
    expect.arrayContaining([1, 2, 3, 4, 5, 6, 7, 8, 9]),
  );
});
