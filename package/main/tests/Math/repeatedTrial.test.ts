import { repeatedTrial } from '../../module/Math/repeatedTrial';
test('{repeatedTrial}', () => {
    expect(repeatedTrial(4, 2, { x: 1, y: 3 })).toBe([8, 27]);
});
