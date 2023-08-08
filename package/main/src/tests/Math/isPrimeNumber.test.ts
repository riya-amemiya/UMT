import { isPrimeNumber } from "@/Math/isPrimeNumber";
test("{isPrimeNumber}", () => {
  expect(isPrimeNumber(1)).toBeFalsy();
  expect(isPrimeNumber(2)).toBeTruthy();
  expect(isPrimeNumber(3)).toBeTruthy();
  expect(isPrimeNumber(4)).toBeFalsy();
  expect(isPrimeNumber(5)).toBeTruthy();
  expect(isPrimeNumber(6)).toBeFalsy();
  expect(isPrimeNumber(100)).toBeFalsy();
});
