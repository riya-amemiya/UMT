import type { Config } from "jest";

const config: Config = {
  transform: {
    "^.+\\.(t|j)sx?$": "@swc/jest",
  },
  testEnvironment: "node",
  roots: ["<rootDir>/src/tests"],
  collectCoverage: true,
  collectCoverageFrom: [
    "**/*.ts",
    "!<rootDir>/**/{birthday,birthdaySimple,isNode,isNodeWebkit}.ts",
  ],
  coveragePathIgnorePatterns: [
    "<rootDir>/src/Date",
    "<rootDir>/src/Simple/Date",
  ],
  coverageDirectory: "./coverage",
  moduleDirectories: ["node_modules", "src"],
  moduleNameMapper: {
    "@/(.*)": "<rootDir>/src/$1",
  },
};
export default config;
