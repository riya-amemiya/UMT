import { literalExpression } from '../../../module/Math/calculator/literalExpression';
test('literalExpression', () => {
    expect(literalExpression(1)).toBe('1');
    expect(literalExpression(2)).toBe('2');
    expect(literalExpression(3)).toBe('3');
});
