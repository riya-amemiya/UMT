export type DeepPartial<T> = T extends object
  ? {
      [P in keyof T]?: DeepPartial<T[P]>;
    }
  : T;

export type UMT_i18nData<
  T extends { [key: string]: string | NestedObject },
  L extends string,
> = {
  [key in L]: DeepPartial<T>;
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
export type FormatterFunction = (value: unknown, locale: string) => string;

export type UMT_i18nOptions = {
  formatters?: Record<string, FormatterFunction>;
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
  private formatters: Map<string, FormatterFunction>;

  constructor(data: T, locale: keyof T, options?: UMT_i18nOptions) {
    this.data = data;
    this.flattenedData = this.flattenAllLocales(data);
    this.locale = locale;
    this.defaultLocale = locale;
    this.formatters = new Map(Object.entries(options?.formatters ?? {}));
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


  public setFallbackLocales(locales: (keyof T)[]): this {
    this.fallbackLocales = locales;
    return this;
  }

  public getFallbackLocales(): (keyof T)[] {
    return this.fallbackLocales;
  }

  public hasTranslation(key: string): boolean {
    return this.getTranslation(key) !== undefined;
  }

  public getAllTranslations(locale?: keyof T): Record<string, string> {
    const targetLocale = locale || this.locale;
    return this.flattenedData[targetLocale] || {};
  }
}
