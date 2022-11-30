import { roundOf } from '../../module/Math/roundOf';
test('{roundOf', () => {
    expect(roundOf(1.1, 1)).toBe(1.1);
});
