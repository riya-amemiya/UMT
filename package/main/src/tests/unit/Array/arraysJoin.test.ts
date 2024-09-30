import { arraysJoin } from "@/Array/arraysJoin";
test("{arraysJoin}", () => {
  expect(arraysJoin([1, 2, 3], [4, 5, 6])).toEqual([1, 2, 3, 4, 5, 6]);
  expect(arraysJoin([1, 2, 3], [4, 5, 6], [7, 8, 9])).toEqual([
    1, 2, 3, 4, 5, 6, 7, 8, 9,
  ]);
});
