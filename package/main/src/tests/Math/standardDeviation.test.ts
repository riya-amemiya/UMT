import { standardDeviation } from "../../../module/Math/standardDeviation";
test("{standardDeviation}", () => {
  expect(standardDeviation([1, 2, 3, 4, 5])).toBe(1.4142135623730951);
});
