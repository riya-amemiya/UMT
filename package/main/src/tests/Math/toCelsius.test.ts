import { toCelsius } from "../../../module/Math/toCelsius";
test("{toCelsius}", () => {
  expect(toCelsius(32)).toBe(-241.15);
});
