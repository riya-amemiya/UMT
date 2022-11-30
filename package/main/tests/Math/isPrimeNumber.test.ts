import { isPrimeNumber } from '../../module/Math/isPrimeNumber';
test('{isPrimeNumber}', () => {
    expect(isPrimeNumber(1)).toBeFalsy();
    expect(isPrimeNumber(2)).toBeTruthy();
    expect(isPrimeNumber(3)).toBeTruthy();
    expect(isPrimeNumber(4)).toBeFalsy();
    expect(isPrimeNumber(5)).toBeTruthy();
});
