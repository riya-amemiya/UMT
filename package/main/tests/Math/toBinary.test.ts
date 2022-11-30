import { toBinary } from '../../module/Math/toBinary';
test('{toBinary}', () => {
    expect(toBinary(1)).toBe('1');
    expect(toBinary(2)).toBe('10');
});
