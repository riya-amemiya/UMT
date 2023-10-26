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
    "!**/{index,random,birthday,isBrowser,isNode,isNodeWebkit,clock}.{js,ts}",
    "!**/Date/**",
  ],
  coverageDirectory: "coverage_dir",
  coverageReporters: ["text"],
  moduleDirectories: ["node_modules", "src"],
  moduleNameMapper: {
    "@/(.*)": "<rootDir>/src/$1",
  },
};
export default config;
