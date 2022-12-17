import { subtract } from '../../module/Math/subtract';
test('{subtract}', () => {
    expect(subtract(1, 1)).toBe(0);
    expect(subtract(1.1, 1)).toBe(0.1);
    expect(subtract(1, 1.1)).toBe(-0.1);
});
