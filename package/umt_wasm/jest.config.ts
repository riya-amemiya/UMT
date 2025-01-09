import type { Config } from "jest";

const config: Config = {
	transform: {
		"^.+\\.(t|j)sx?$": "@swc/jest",
	},
	testEnvironment: "node",
	roots: ["<rootDir>/tests"],
	cacheDirectory: "<rootDir>/node_modules/.cache/jest",
	moduleDirectories: ["node_modules", "src"],
	moduleNameMapper: {
		"@/(.*)": "<rootDir>/pkg/$1",
	},
};
export default config;
