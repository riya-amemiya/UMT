import eslint from "@eslint/js";
import tseslint from "typescript-eslint";
import unicornPlugin from "eslint-plugin-unicorn";
import importPlugin from "eslint-plugin-import";
import baselineJs, { BASELINE } from "eslint-plugin-baseline-js";
import { defineConfig } from "eslint/config";

export default defineConfig(
  eslint.configs.recommended,
  tseslint.configs.strict,
  tseslint.configs.stylisticTypeChecked,
  unicornPlugin.configs.recommended,
  baselineJs.configs["recommended-ts"]({
    available: BASELINE.WIDELY,
  }),
  {
    ignores: [
      "jest.config.ts",
      "tmp/",
      "src/tests/",
      ".dependency-cruiser.js",
      "cjs.build.mjs",
      "eslint.config.mjs",
    ],
  },
  {
    languageOptions: {
      sourceType: "module",
      ecmaVersion: 2024,
      parserOptions: {
        project: "./tsconfig.json",
      },
    },
    plugins: {
      "baseline-js": baselineJs,
      import: importPlugin,
    },
    rules: {
      "@typescript-eslint/no-explicit-any": "off",
      "@typescript-eslint/no-unused-vars": "off",
      "@typescript-eslint/no-non-null-assertion": "off",
      "@typescript-eslint/unified-signatures": "off",
      "@typescript-eslint/consistent-indexed-object-style": "off",
      "@typescript-eslint/no-unnecessary-type-assertion": [
        "error",
        { checkLiteralConstAssertions: true },
      ],
      "no-constant-condition": "off",
      "unicorn/filename-case": [
        "error",
        {
          cases: {
            camelCase: true,
            pascalCase: true,
          },
        },
      ],
      "unicorn/no-null": "off",
      "unicorn/prefer-spread": "off",
      "unicorn/no-array-reduce": "off",
      "unicorn/prefer-module": "off",
      "unicorn/prefer-number-properties": "off",
      "unicorn/no-nested-ternary": "off",
      "unicorn/number-literal-case": "off",
      "unicorn/no-array-reverse": "off",
      "import/order": [
        "error",
        {
          groups: [
            "builtin",
            "external",
            "internal",
            "parent",
            "sibling",
            "index",
          ],
          alphabetize: {
            order: "asc",
          },
          "newlines-between": "always",
        },
      ],
    },
  },
);
