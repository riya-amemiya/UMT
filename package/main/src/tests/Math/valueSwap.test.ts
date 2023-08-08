import { valueSwap } from "../../../module/Math/valueSwap";
test("{valueSwap}", () => {
  expect(valueSwap(1, 2)).toEqual(expect.arrayContaining([2, 1]));
});
