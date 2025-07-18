import type { Config } from "jest";

const config: Config = {
  transform: {
    "^.+\\.(t|j)sx?$": "@swc/jest",
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
};

export default config;
