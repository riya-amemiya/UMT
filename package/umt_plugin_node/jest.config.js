/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports = {
    preset: 'ts-jest',
    testEnvironment: 'node',
    roots: ['<rootDir>/tests'],
    collectCoverage: true,
    collectCoverageFrom: [
        '<rootDir>/tests/**/*.ts',
        '!**/node_modules/**',
    ],
    coverageDirectory: 'coverage_dir',
    coverageReporters: ['html'],
};
