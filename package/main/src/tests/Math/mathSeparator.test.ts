import { mathSeparator } from "../../../module/Math/mathSeparator";
test("{mathSeparator}", () => {
  expect(mathSeparator("1100")).toEqual(expect.arrayContaining([1000, 100]));
});
