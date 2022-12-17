import { toBinary } from '../../module/Math/toBinary';
test('{toBinary}', () => {
    expect(toBinary(1)).toBe('1');
    expect(toBinary(2)).toBe('10');
    expect(toBinary(3)).toBe('11');
    expect(toBinary(4)).toBe('100');
    expect(toBinary(5)).toBe('101');
    expect(toBinary(6)).toBe('110');
    expect(toBinary(7, 4)).toBe('13');
});
