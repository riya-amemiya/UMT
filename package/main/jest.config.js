/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports = {
    preset: 'ts-jest',
    testEnvironment: 'node',
    roots: ['<rootDir>/tests'],
    collectCoverage: true,
    collectCoverageFrom: [
        '<rootDir>/module/**/*.{js,ts}',
        '<rootDir>/src/**/*.{js,ts}',
        '!**/node_modules/**',
        '!**/{index,random}.{js,ts}',
        '!**/Date/**',
    ],
    coverageDirectory: 'coverage_dir',
    coverageReporters: ['text'],
};
