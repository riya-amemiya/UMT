import { lcm } from "../../module/Math/lcm";
test("{lcm}", () => {
  expect(lcm(1, 1)).toBe(1);
  expect(lcm(1, 2)).toBe(2);
  expect(lcm(2, 1)).toBe(2);
  expect(lcm(2, 2)).toBe(2);
  expect(lcm(2, 3)).toBe(6);
});
