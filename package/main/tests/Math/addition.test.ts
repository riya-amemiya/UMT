import { addition } from '../../module/Math/addition';
test('{addition}', () => {
    expect(addition(1, 2)).toBe(3);
    expect(addition(1.1, 2.2)).toBe(3.3);
});
