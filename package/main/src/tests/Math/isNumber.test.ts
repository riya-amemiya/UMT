import { isNumber } from "../../../module/Math/isNumber";
test("{isNumber}", () => {
  expect(isNumber(1)).toBeTruthy();
  expect(isNumber(1.1)).toBeTruthy();
  expect(isNumber(1.11)).toBeTruthy();
  expect(isNumber(1.111)).toBeTruthy();
  expect(isNumber("1.1111")).toBeTruthy();
  expect(isNumber("1.11111", false)).toBeFalsy();
});
