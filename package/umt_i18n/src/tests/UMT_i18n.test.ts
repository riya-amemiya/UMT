import { UMT_i18n, type UMT_i18nData } from "@/umtI18n";

const data: UMT_i18nData<
  {
    hello: string;
    world: string;
  },
  "en" | "zh" | "ja"
> = {
  en: {
    hello: "hello",
    world: "world",
  },
  zh: {
    hello: "你好",
    world: "世界",
  },
  ja: {
    hello: "こんにちは",
  },
};

const advancedData: UMT_i18nData<
  {
    greeting: string;
    items: string;
    items_zero: string;
    items_one: string;
    items_other: string;
    nested: {
      deep: {
        message: string;
      };
    };
    date: string;
    price: string;
  },
  "en" | "ja" | "fr"
> = {
  en: {
    greeting: "Hello {{name}}!",
    items_zero: "No items",
    items_one: "1 item",
    items_other: "{{count}} items",
    nested: {
      deep: {
        message: "This is a deeply nested message",
      },
    },
    date: "Today is {{date}}",
    price: "Price: {{currency}}",
  },
  ja: {
    greeting: "こんにちは {{name}}さん!",
    items: "{{count}}個のアイテム",
    nested: {
      deep: {
        message: "これはネストされたメッセージです",
      },
    },
    date: "今日は{{date}}です",
    price: "価格: {{currency}}",
  },
  fr: {
    greeting: "Bonjour {{name}}!",
    items_zero: "Aucun article",
    items_one: "1 article",
    items_other: "{{count}} articles",
  },
};

const i18n = new UMT_i18n(data, "en");
describe("UMT_i18n Basic Tests", () => {
  test("Basic translation", () => {
    expect(i18n.getLocale()).toBe("en");
    expect(i18n.setLocale("zh").translate("hello")).toBe("你好");
    expect(i18n.setLocale("en").translate("hello")).toBe("hello");
    expect(i18n.setLocale("zh").translate("world")).toBe("世界");
    expect(i18n.setLocale("en").translate("world")).toBe("world");
    expect(i18n.getLocaleData()).toBe(data);
    expect(i18n.getDefaultLocale()).toBe("en");
    expect(i18n.setLocale("ja").translate("hello")).toBe("こんにちは");
    expect(i18n.translate("world")).toBe("world");
    i18n.setDefaultLocale("ja");
    expect(i18n.translate("world")).toBe("world");
  });

  test("Translation with defaultValue", () => {
    const i18nBasic = new UMT_i18n(data, "en");
    expect(
      // biome-ignore lint/suspicious/noExplicitAny: ignore this for testing purposes
      i18nBasic.translate("notExist" as any, { defaultValue: "Default" }),
    ).toBe("Default");
    // biome-ignore lint/suspicious/noExplicitAny: ignore this for testing purposes
    expect(i18nBasic.translate("notExist" as any)).toBe("notExist");
  });

  test("t() alias", () => {
    const i18nBasic = new UMT_i18n(data, "en");
    expect(i18nBasic.t("hello")).toBe("hello");
    expect(i18nBasic.setLocale("ja").t("hello")).toBe("こんにちは");
  });
});

describe("UMT_i18n Advanced Features", () => {
  const advI18n = new UMT_i18n(advancedData, "en");

  test("Placeholder replacement", () => {
    expect(advI18n.translate("greeting", { params: { name: "John" } })).toBe(
      "Hello John!",
    );
    advI18n.setLocale("ja");
    expect(advI18n.translate("greeting", { params: { name: "太郎" } })).toBe(
      "こんにちは 太郎さん!",
    );
  });

  test("Plural support", () => {
    advI18n.setLocale("en");
    expect(advI18n.translate("items", { count: 0 })).toBe("No items");
    expect(advI18n.translate("items", { count: 1 })).toBe("1 item");
    expect(advI18n.translate("items", { count: 5, params: { count: 5 } })).toBe(
      "5 items",
    );

    advI18n.setLocale("ja");
    expect(advI18n.translate("items", { count: 0, params: { count: 0 } })).toBe(
      "0個のアイテム",
    );
    expect(advI18n.translate("items", { count: 5, params: { count: 5 } })).toBe(
      "5個のアイテム",
    );
  });

  test("Nested key support", () => {
    advI18n.setLocale("en");
    expect(advI18n.translate("nested.deep.message")).toBe(
      "This is a deeply nested message",
    );
    advI18n.setLocale("ja");
    expect(advI18n.translate("nested.deep.message")).toBe(
      "これはネストされたメッセージです",
    );
  });

  test("Fallback chain", () => {
    advI18n.setLocale("fr");
    advI18n.setFallbackLocales(["en", "ja"]);
    expect(advI18n.translate("nested.deep.message")).toBe(
      "This is a deeply nested message",
    );

    advI18n.setFallbackLocales(["ja", "en"]);
    expect(advI18n.translate("date", { params: { date: "2024-01-01" } })).toBe(
      "今日は2024-01-01です",
    );

    expect(advI18n.getFallbackLocales()).toEqual(["ja", "en"]);
  });

  test("Custom formatter in constructor", () => {
    const customI18n = new UMT_i18n(advancedData, "en", {
      formatters: {
        uppercase: (value) => String(value).toUpperCase(),
        lowercase: (value) => String(value).toLowerCase(),
      },
    });

    customI18n.setLocale("en");
    const customData = {
      ...advancedData,
      en: {
        ...advancedData.en,
        customUpper: "Hello {{uppercase}}!",
        customLower: "Hello {{lowercase}}!",
      },
    };
    const testI18n = new UMT_i18n(customData, "en", {
      formatters: {
        uppercase: (value) => String(value).toUpperCase(),
        lowercase: (value) => String(value).toLowerCase(),
      },
    });

    expect(
      testI18n.translate("customUpper", { params: { uppercase: "world" } }),
    ).toBe("Hello WORLD!");
    expect(
      testI18n.translate("customLower", { params: { lowercase: "WORLD" } }),
    ).toBe("Hello world!");
  });

  test("hasTranslation", () => {
    advI18n.setLocale("en");
    expect(advI18n.hasTranslation("greeting")).toBe(true);
    // biome-ignore lint/suspicious/noExplicitAny: ignore this for testing purposes
    expect(advI18n.hasTranslation("notExist" as any)).toBe(false);
    expect(advI18n.hasTranslation("nested.deep.message")).toBe(true);
  });

  test("getAllTranslations", () => {
    const enTranslations = advI18n.getAllTranslations("en");
    // biome-ignore lint/complexity/useLiteralKeys: TypeScript requires bracket notation for index signatures
    expect(enTranslations["greeting"]).toBe("Hello {{name}}!");
    expect(enTranslations["nested.deep.message"]).toBe(
      "This is a deeply nested message",
    );

    const frTranslations = advI18n.getAllTranslations("fr");
    // biome-ignore lint/complexity/useLiteralKeys: TypeScript requires bracket notation for index signatures
    expect(frTranslations["greeting"]).toBe("Bonjour {{name}}!");
  });

  test("Nested object with null values", () => {
    const testI18n = new UMT_i18n(
      {
        en: {
          test: {
            value: null as unknown as string,
            string: "test",
          },
        },
      },
      "en",
    );

    expect(testI18n.translate("test.string")).toBe("test");
    expect(testI18n.translate("test.value")).toBe("test.value");
  });

  test("getAllTranslations with non-existent locale", () => {
    const testI18n = new UMT_i18n({ en: { test: "test" } }, "en");
    const result = testI18n.getAllTranslations(
      "nonexistent" as keyof { en: { test: string } },
    );
    expect(result).toEqual({});
  });

  test("getAllTranslations without locale parameter uses current locale", () => {
    advI18n.setLocale("ja");
    const result = advI18n.getAllTranslations();
    // biome-ignore lint/complexity/useLiteralKeys: TypeScript requires bracket notation for index signatures
    expect(result["greeting"]).toBe("こんにちは {{name}}さん!");
    // biome-ignore lint/complexity/useLiteralKeys: TypeScript requires bracket notation for index signatures
    expect(result["items"]).toBe("{{count}}個のアイテム");
  });

  test("Placeholder with undefined parameter key", () => {
    advI18n.setLocale("en");
    const testData = {
      en: {
        test: "Hello {{name}} {{age}}!",
      },
    };
    const testI18n = new UMT_i18n(testData, "en");
    expect(testI18n.translate("test", { params: { name: "John" } })).toBe(
      "Hello John {{age}}!",
    );
  });

  test("supports deeply nested objects with partial properties", () => {
    type NestedData = {
      menu: {
        file: {
          new: string;
          open: string;
          save: string;
        };
        edit: {
          cut: string;
          copy: string;
          paste: string;
        };
      };
    };

    const nestedData: UMT_i18nData<NestedData, "en" | "ja"> = {
      en: {
        menu: {
          file: {
            new: "New",
          },
        },
      },
      ja: {
        menu: {
          file: {
            new: "新規",
            open: "開く",
          },
          edit: {
            cut: "切り取り",
          },
        },
      },
    };

    const i18n = new UMT_i18n(nestedData, "en");
    expect(i18n.translate("menu.file.new")).toBe("New");
    expect(i18n.translate("menu.file.open")).toBe("menu.file.open");

    i18n.setLocale("ja");
    expect(i18n.translate("menu.file.new")).toBe("新規");
    expect(i18n.translate("menu.file.open")).toBe("開く");
    expect(i18n.translate("menu.edit.cut")).toBe("切り取り");
    expect(i18n.translate("menu.edit.copy")).toBe("menu.edit.copy");
  });
});
