import { mathSeparator } from '../../module/Math/mathSeparator';
test('{mathSeparator}', () => {
    expect(mathSeparator('1100')).toBe([1000, 100]);
});
