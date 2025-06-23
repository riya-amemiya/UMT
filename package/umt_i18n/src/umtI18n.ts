export type UMT_i18nData<
  T extends { [key: string]: string },
  L extends string,
> = {
  [key in L]: Partial<T>;
};

export type TranslateOptions = {
  params?: Record<string, string | number | Date | unknown>;
  count?: number;
  defaultValue?: string;
};

export type NestedObject = {
  [key: string]: string | NestedObject;
};

export type FlattenedData<T> = {
  [K in keyof T]: Record<string, string>;
};
export class UMT_i18n<
  T extends {
    [key: string]: {
      [k: string]: string | NestedObject;
    };
  },
> {
  private data: T;
  private flattenedData: FlattenedData<T>;
  private locale: keyof T;
  private defaultLocale: keyof T;
  private fallbackLocales: (keyof T)[] = [];
  private formatters: Map<string, (value: unknown, locale: string) => string> =
    new Map();

  constructor(data: T, locale: keyof T) {
    this.data = data;
    this.flattenedData = this.flattenAllLocales(data);
    this.locale = locale;
    this.defaultLocale = locale;
    this.initializeDefaultFormatters();
  }
  public getLocale() {
    return this.locale;
  }
  public setLocale(locale: keyof T) {
    this.locale = locale;
    return this;
  }

  public getLocaleData() {
    return this.data;
  }

  public getDefaultLocale() {
    return this.defaultLocale;
  }

  public setDefaultLocale(locale: keyof T) {
    this.defaultLocale = locale;
    return this;
  }

  public translate(key: string, options?: TranslateOptions): string {
    const { params, count, defaultValue } = options || {};

    let translation = this.getTranslation(key, count);

    if (!translation) {
      return defaultValue || key;
    }

    if (params) {
      translation = this.replacePlaceholders(translation, params);
    }

    return translation;
  }

  public t(key: string, options?: TranslateOptions): string {
    return this.translate(key, options);
  }

  private getTranslation(key: string, count?: number): string | undefined {
    const actualKey = count === undefined ? key : this.getPluralKey(key, count);

    let translation = this.flattenedData[this.locale]?.[actualKey];

    if (!translation) {
      for (const fallbackLocale of this.fallbackLocales) {
        translation = this.flattenedData[fallbackLocale]?.[actualKey];
        if (translation) {
          break;
        }
      }
    }

    if (!translation) {
      translation = this.flattenedData[this.defaultLocale]?.[actualKey];
    }

    return translation;
  }

  private getPluralKey(key: string, count: number): string {
    const locale = String(this.locale);
    const pluralSuffix = this.getPluralSuffix(count, locale);
    return `${key}${pluralSuffix}`;
  }

  private getPluralSuffix(count: number, locale: string): string {
    if (locale === "ja" || locale === "zh" || locale === "ko") {
      return "";
    }

    if (count === 0) {
      return "_zero";
    }
    if (count === 1) {
      return "_one";
    }
    return "_other";
  }

  private replacePlaceholders(
    text: string,
    parameters: Record<string, string | number | Date | unknown>,
  ): string {
    return text.replaceAll(/\{\{(\w+)\}\}/g, (match, key) => {
      const value = parameters[key];
      if (value === undefined) {
        return match;
      }

      const formatter = this.formatters.get(key);
      if (formatter) {
        return formatter(value, String(this.locale));
      }

      return String(value);
    });
  }

  private flattenObject(
    object: NestedObject,
    prefix = "",
  ): Record<string, string> {
    const result: Record<string, string> = {};

    for (const [key, value] of Object.entries(object)) {
      const newKey = prefix ? `${prefix}.${key}` : key;

      if (typeof value === "string") {
        result[newKey] = value;
      } else if (typeof value === "object" && value !== null) {
        Object.assign(
          result,
          this.flattenObject(value as NestedObject, newKey),
        );
      }
    }

    return result;
  }

  private flattenAllLocales(data: T): FlattenedData<T> {
    const flattened = {} as FlattenedData<T>;

    for (const [locale, localeData] of Object.entries(data)) {
      (flattened as Record<keyof T, Record<string, string>>)[
        locale as keyof T
      ] = this.flattenObject(localeData as NestedObject);
    }

    return flattened;
  }

  private initializeDefaultFormatters(): void {
    this.formatters.set("date", (value, locale) => {
      if (value instanceof Date) {
        return value.toLocaleDateString(locale);
      }
      return String(value);
    });

    this.formatters.set("time", (value, locale) => {
      if (value instanceof Date) {
        return value.toLocaleTimeString(locale);
      }
      return String(value);
    });

    this.formatters.set("number", (value, locale) => {
      if (typeof value === "number") {
        return value.toLocaleString(locale);
      }
      return String(value);
    });

    this.formatters.set("currency", (value, locale) => {
      if (typeof value === "number") {
        const currencyMap: Record<string, string> = {
          en: "USD",
          ja: "JPY",
          zh: "CNY",
          de: "EUR",
          fr: "EUR",
        };
        const currency = currencyMap[locale] || "USD";
        return new Intl.NumberFormat(locale, {
          style: "currency",
          currency,
        }).format(value);
      }
      return String(value);
    });
  }

  public setFallbackLocales(locales: (keyof T)[]): this {
    this.fallbackLocales = locales;
    return this;
  }

  public getFallbackLocales(): (keyof T)[] {
    return this.fallbackLocales;
  }

  public addFormatter(
    key: string,
    formatter: (value: unknown, locale: string) => string,
  ): this {
    this.formatters.set(key, formatter);
    return this;
  }

  public hasTranslation(key: string): boolean {
    return this.getTranslation(key) !== undefined;
  }

  public getAllTranslations(locale?: keyof T): Record<string, string> {
    const targetLocale = locale || this.locale;
    return this.flattenedData[targetLocale] || {};
  }

  public addTranslations(
    locale: keyof T,
    translations: Record<string, string | NestedObject>,
  ): this {
    if (!this.data[locale]) {
      this.data[locale] = {} as T[keyof T];
    }

    Object.assign(this.data[locale], translations);
    this.flattenedData = this.flattenAllLocales(this.data);
    return this;
  }

  public removeTranslation(locale: keyof T, key: string): this {
    if (this.data[locale]) {
      delete (this.data[locale] as Record<string, string | NestedObject>)[key];
      this.flattenedData = this.flattenAllLocales(this.data);
    }
    return this;
  }
}
