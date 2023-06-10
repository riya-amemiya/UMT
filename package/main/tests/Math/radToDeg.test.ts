import { radToDeg } from "../../module/Math/radToDeg";
test("{radToDeg}", () => {
  expect(radToDeg(1)).toBe(57.29577951308232);
  expect(radToDeg(2)).toBe(114.59155902616465);
  expect(radToDeg(3)).toBe(171.88733853924697);
});
