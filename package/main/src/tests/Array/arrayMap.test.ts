import { arrayMap } from "@/Array/arrayMap";

test("{arrayMap}", () => {
  expect(arrayMap([1, 2, 3], (value) => value * 2)).toEqual(
    expect.arrayContaining([2, 4, 6]),
  );
  expect(arrayMap([1, 2, 3], (value) => value * 2)).toHaveLength(3);
  expect(arrayMap([1, 2, 3], (value) => value * 2)).not.toContain(1);
  expect(arrayMap([1, 2, 3], (value) => value * 2)).not.toContain(3);
});
