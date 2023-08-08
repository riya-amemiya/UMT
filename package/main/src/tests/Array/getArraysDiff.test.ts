import { getArraysDiff } from "../../../module/Array/getArraysDiff";
test("{getArraysDiff}", () => {
  expect(getArraysDiff([1, 2, 3], [2, 3, 4], [3, 4, 5])).toEqual(
    expect.arrayContaining([1, 5]),
  );
});
