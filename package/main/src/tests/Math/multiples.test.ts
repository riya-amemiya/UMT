import { multiples } from "../../../module/Math/multiples";
test("{multiples}", () => {
  expect(multiples(1, 1)).toEqual(expect.arrayContaining([1]));
  expect(multiples(1, 2)).toEqual(expect.arrayContaining([1, 2]));
});
