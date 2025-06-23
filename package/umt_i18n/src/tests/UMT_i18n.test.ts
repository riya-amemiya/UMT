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

const advancedData = {
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
    expect(i18nBasic.translate("notExist", { defaultValue: "Default" })).toBe(
      "Default",
    );
    expect(i18nBasic.translate("notExist")).toBe("notExist");
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

  test("Format functions", () => {
    advI18n.setLocale("en");
    const date = new Date("2024-01-01");
    expect(advI18n.translate("date", { params: { date } })).toBe(
      "Today is 1/1/2024",
    );

    const price = 1000;
    expect(advI18n.translate("price", { params: { currency: price } })).toBe(
      "Price: $1,000.00",
    );

    advI18n.setLocale("ja");
    expect(advI18n.translate("price", { params: { currency: price } })).toBe(
      "価格: ￥1,000",
    );
  });

  test("Custom formatter", () => {
    advI18n.addFormatter("uppercase", (value) => String(value).toUpperCase());
    advI18n.setLocale("en");
    advI18n.addTranslations("en", { custom: "Hello {{uppercase}}!" });
    expect(
      advI18n.translate("custom", { params: { uppercase: "world" } }),
    ).toBe("Hello WORLD!");
  });

  test("hasTranslation", () => {
    advI18n.setLocale("en");
    expect(advI18n.hasTranslation("greeting")).toBe(true);
    expect(advI18n.hasTranslation("notExist")).toBe(false);
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

  test("addTranslations", () => {
    advI18n.addTranslations("en", {
      newKey: "New translation",
      another: {
        nested: "Another nested",
      },
    });

    expect(advI18n.translate("newKey")).toBe("New translation");
    expect(advI18n.translate("another.nested")).toBe("Another nested");
  });

  test("removeTranslation", () => {
    advI18n.setLocale("en");
    advI18n.addTranslations("en", { toRemove: "Will be removed" });
    expect(advI18n.hasTranslation("toRemove")).toBe(true);

    advI18n.removeTranslation("en", "toRemove");
    expect(advI18n.hasTranslation("toRemove")).toBe(false);
  });

  test("Edge cases for formatters", () => {
    advI18n.setLocale("en");

    advI18n.addTranslations("en", { dateTest: "Date: {{date}}" });
    expect(
      advI18n.translate("dateTest", { params: { date: "2024-01-01" } }),
    ).toBe("Date: 2024-01-01");

    advI18n.addTranslations("en", { timeTest: "Time: {{time}}" });
    expect(advI18n.translate("timeTest", { params: { time: "12:00" } })).toBe(
      "Time: 12:00",
    );

    advI18n.addTranslations("en", { numberTest: "Number: {{number}}" });
    expect(
      advI18n.translate("numberTest", { params: { number: "1000" } }),
    ).toBe("Number: 1000");

    advI18n.addTranslations("en", { currencyTest: "Currency: {{currency}}" });
    expect(
      advI18n.translate("currencyTest", { params: { currency: "$1000" } }),
    ).toBe("Currency: $1000");
  });

  test("Formatter without registered handler", () => {
    advI18n.setLocale("en");
    advI18n.addTranslations("en", { test: "Value: {{unknownFormatter}}" });
    expect(
      advI18n.translate("test", { params: { unknownFormatter: "value" } }),
    ).toBe("Value: value");
  });

  test("removeTranslation with non-existent locale", () => {
    const testI18n = new UMT_i18n({ en: { test: "test" } }, "en");
    testI18n.removeTranslation("fr" as keyof { en: { test: string } }, "test");
    expect(testI18n.hasTranslation("test")).toBe(true);
  });

  test("Currency formatter with unknown locale", () => {
    const testI18n = new UMT_i18n({ unknown: {} }, "unknown");
    testI18n.addTranslations("unknown", { price: "Price: {{currency}}" });
    const price = 1000;
    expect(
      testI18n.translate("price", { params: { currency: price } }),
    ).toMatch(/1,000/);
  });

  test("Placeholder with undefined value", () => {
    advI18n.setLocale("en");
    advI18n.addTranslations("en", { test: "Hello {{name}}!" });
    expect(advI18n.translate("test", { params: {} })).toBe("Hello {{name}}!");
  });

  test("Date formatter with actual Date objects", () => {
    advI18n.setLocale("en");
    advI18n.addTranslations("en", {
      dateFormat: "Date: {{date}}",
      timeFormat: "Time: {{time}}",
    });

    const testDate = new Date("2024-01-15T10:30:00");
    expect(
      advI18n.translate("dateFormat", { params: { date: testDate } }),
    ).toBe("Date: 1/15/2024");
    expect(
      advI18n.translate("timeFormat", { params: { time: testDate } }),
    ).toMatch(/Time: \d{1,2}:\d{2}:\d{2}/);
  });

  test("Number formatter with actual numbers", () => {
    advI18n.setLocale("en");
    advI18n.addTranslations("en", { numberFormat: "Number: {{number}}" });

    expect(
      advI18n.translate("numberFormat", { params: { number: 1234.56 } }),
    ).toBe("Number: 1,234.56");
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

  test("addTranslations to new locale", () => {
    type TestData = { en: { hello: string }; de?: Record<string, string> };
    const testI18n = new UMT_i18n<TestData>({ en: { hello: "Hello" } }, "en");
    testI18n.addTranslations("de" as keyof TestData, {
      hello: "Hallo",
      world: "Welt",
    });
    testI18n.setLocale("de" as keyof TestData);
    expect(testI18n.translate("hello")).toBe("Hallo");
    expect(testI18n.translate("world")).toBe("Welt");
  });
});
