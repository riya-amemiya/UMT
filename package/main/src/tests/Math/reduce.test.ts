import { reduce } from "@/Math/reduce";
test("{reduce}", () => {
  expect(reduce(16, 2)).toMatchObject({
    x: 8,
    y: 1,
    gcd: 2,
  });
  expect(reduce(0, 2)).toMatchObject({
    x: NaN,
    y: NaN,
  });
});
