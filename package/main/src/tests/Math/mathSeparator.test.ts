import { mathSeparator } from "@/Math/mathSeparator";
test("{mathSeparator}", () => {
  expect(mathSeparator("1100")).toEqual([1000, 100]);
  expect(mathSeparator("1110")).toEqual([1000, 110]);
  expect(mathSeparator("1111")).toEqual([1000, 111]);
  expect(mathSeparator("11110")).toEqual([10000, 1110]);
  expect(mathSeparator("11111")).toEqual([10000, 1111]);
  expect(mathSeparator("111110")).toEqual([100000, 11110]);
});
