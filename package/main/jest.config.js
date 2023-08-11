/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports = {
  transform: {
    "^.+\\.(t|j)sx?$": ["ts-jest"],
  },
  testEnvironment: "node",
  roots: ["<rootDir>/src/tests"],
  collectCoverage: true,
  collectCoverageFrom: [
    "!**/node_modules/**",
    "!**/{index,random}.{js,ts}",
    "!**/Date/**",
  ],
  coverageDirectory: "coverage_dir",
  coverageReporters: ["text"],
  moduleDirectories: ["node_modules", "src"],
  moduleNameMapper: {
    "@/(.*)": "<rootDir>/src/$1",
  },
};
