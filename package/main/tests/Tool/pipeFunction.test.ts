import { pipeFunction } from '../../module/Tool/pipeFunction';
test('pipeFunction', () => {
    expect(pipeFunction(3)((x) => x + 1)((x) => x * 2)()).toBe(8);
    expect(
        pipeFunction(3)((x) => x + 1)((x) => x * 2)((x) => x - 1)(),
    ).toBe(7);
});
