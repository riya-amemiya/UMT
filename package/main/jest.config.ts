import type { Config } from "jest";

const config: Config = {
  transform: {
    "^.+\\.(t|j)sx?$": "@swc/jest",
  },
  testEnvironment: "node",
  roots: ["<rootDir>/src/tests"],
  cacheDirectory: "<rootDir>/node_modules/.cache/jest",
  collectCoverage: true,
  collectCoverageFrom: [
    "<rootDir>/src/**/*.ts",
    "!<rootDir>/**/{isBrowser,isNode,isNodeWebkit,isBun}.ts",
    "!<rootDir>/src/tests/benchmark/**/*.ts",
  ],
  coverageDirectory: "./coverage",
  moduleDirectories: ["node_modules", "src"],
  moduleNameMapper: {
    "@/(.*)": "<rootDir>/src/$1",
  },
};
export default config;
