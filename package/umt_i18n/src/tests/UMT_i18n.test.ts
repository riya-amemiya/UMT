import { UMT_i18nData, UMT_i18n } from "..";
const data: UMT_i18nData<
  {
    hello: string;
    world: string;
  },
  "en" | "zh"
> = {
  en: {
    hello: "hello",
    world: "world",
  },
  zh: {
    hello: "你好",
    world: "世界",
  },
};

const i18n = new UMT_i18n(data, "en");
test("UMT_i18n Test", () => {
  expect(i18n.getLocale()).toBe("en");
  expect(i18n.setLocale("zh").translate("hello")).toBe("你好");
  expect(i18n.setLocale("en").translate("hello")).toBe("hello");
  expect(i18n.setLocale("zh").translate("world")).toBe("世界");
  expect(i18n.setLocale("en").translate("world")).toBe("world");
  expect(i18n.getLocaleData()).toBe(data);
});
