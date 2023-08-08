import { min } from "../../../module/Math/min";
test("{min}", () => {
  expect(min(1, 1)).toBe(1);
  expect(min(1, 2)).toBe(1);
});
