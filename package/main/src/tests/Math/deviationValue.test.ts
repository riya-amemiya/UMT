import { deviationValue } from "../../../module/Math/deviationValue";
test("{deviationValue}", () => {
  expect(deviationValue(100, 50, 10)).toBe(100);
});
