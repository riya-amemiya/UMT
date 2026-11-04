import js from "@eslint/js";
import typescriptEslintPlugin from "@typescript-eslint/eslint-plugin";
import unicornPlugin from "eslint-plugin-unicorn";
import importPlugin from "eslint-plugin-import";

export default [
  js.configs.recommended,
  unicornPlugin.configs.recommended,
  ...typescriptEslintPlugin.configs["flat/recommended"],
  {
    ignores: [
      "jest.config.ts",
      "tmp/",
      "src/tests/",
      ".dependency-cruiser.js",
      "eslint.config.mjs",
    ],
  },
  {
    languageOptions: {
      ecmaVersion: 2024,
      parserOptions: {
        project: "./tsconfig.json",
      },
    },
    plugins: {
      import: importPlugin,
    },
    rules: {
      "unicorn/filename-case": [
        "error",
        {
          cases: {
            camelCase: true,
            pascalCase: true,
          },
        },
      ],
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
      "unicorn/prefer-includes-over-repeated-comparisons": "off",
    },
  },
];
