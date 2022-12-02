import { check } from '../../module/File/check';
test('Check if file exists', () => {
    expect(check('tests/File/check.test.ts')).toBe(true);
    expect(check('tests/File/check.test.tsx')).toBe(false);
});
