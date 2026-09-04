# umt-i18n

Nested-key i18n helper for TypeScript. This is a **separate** package from [`umt`](../main), not a port of the main utility library. It depends on `umt` only for shared object types (`DeepPartial`, `PickDeepKey`, …).

npm name: `umt-i18n`. Breaking changes may happen without a major bump.

## Install

```bash
npm install umt-i18n
# or
yarn add umt-i18n
# or
pnpm add umt-i18n
# or
bun add umt-i18n
```

## Quick start

```ts
import { UMT_i18n } from "umt-i18n";

const data = {
  en: {
    greeting: "Hello {{name}}!",
    items_zero: "No items",
    items_one: "1 item",
    items_other: "{{count}} items",
    nested: { deep: { message: "Nested" } },
  },
  ja: {
    greeting: "こんにちは {{name}}さん!",
    items: "{{count}}個のアイテム",
  },
};

const i18n = new UMT_i18n(data, "en");
i18n.translate("greeting", { params: { name: "John" } }); // "Hello John!"
i18n.t("nested.deep.message"); // "Nested"
i18n.setLocale("ja").translate("greeting", { params: { name: "太郎" } });
// "こんにちは 太郎さん!"
```

`t()` is an alias of `translate()`.

## Lookup and fallback

Constructor `(data, locale, options?)` sets both the current locale and the default locale.

Resolution order for a missing key:

1. Current locale (`getLocale()` / `setLocale`)
2. `setFallbackLocales([...])` in array order
3. Default locale (`getDefaultLocale()` / `setDefaultLocale`)
4. `options.defaultValue`, or the key string itself

```ts
i18n.setLocale("fr");
i18n.setFallbackLocales(["en", "ja"]);
i18n.translate("nested.deep.message"); // English string, then Japanese, then default
i18n.translate("missing", { defaultValue: "Fallback" }); // "Fallback"
i18n.translate("missing"); // "missing"
```

`hasTranslation(key)` is true when the lookup chain finds a string. `getAllTranslations(locale?)` returns the flattened map for that locale (current locale if omitted). Unknown locales return `{}`.

Keys are flattened with `.` (`nested.deep.message`). Partial locale trees are allowed; a missing nested key falls through the chain above.

## Placeholders and formatters

Placeholders are `{{name}}` (double braces, word characters only). Missing params leave the token unchanged (`"Hello John {{age}}!"`).

Optional `formatters` in the constructor are keyed by **placeholder name**, not by type:

```ts
const i18n = new UMT_i18n(
  { en: { customUpper: "Hello {{uppercase}}!" } },
  "en",
  { formatters: { uppercase: (value) => String(value).toUpperCase() } },
);
i18n.translate("customUpper", { params: { uppercase: "world" } }); // "Hello WORLD!"
```

## Plurals

Pass `count` to `translate`. For locales other than `ja` / `zh` / `ko`, the looked-up key is `${key}_zero` (count 0), `${key}_one` (count 1), or `${key}_other`. Those CJK locales use the key as-is (no suffix).

```ts
i18n.setLocale("en");
i18n.translate("items", { count: 0 }); // "No items"
i18n.translate("items", { count: 1 }); // "1 item"
i18n.translate("items", { count: 5, params: { count: 5 } }); // "5 items"

i18n.setLocale("ja");
i18n.translate("items", { count: 5, params: { count: 5 } }); // "5個のアイテム"
```

## Constraints

- Not a drop-in for `i18next` / `FormatJS`. No ICU message format.
- Depends on `umt` (types). Main `umt` v5 is ESM-only; this package still has an optional Babel CJS build (`bun run build:babel`).
- `setLocale` does not change the default locale.

## Development

```bash
cd package/umt_i18n
bun install
bun run test      # Jest + SWC (`src/tests`)
bun run lint
bun run build     # tsc + tsc-alias → `module/`
```

## License

MIT License
