export type UMT_i18nData<
  T extends { [key: string]: string },
  L extends string,
> = {
  [key in L]: Partial<T>;
};
export class UMT_i18n<
  T extends {
    [key: string]: {
      [k: string]: string;
    };
  },
> {
  private data: T;
  private locale: keyof T;
  private defaultLocale: keyof T;
  constructor(data: T, locale: keyof T) {
    this.data = data;
    this.locale = locale;
    this.defaultLocale = locale;
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

  public translate(key: keyof T[keyof T]) {
    const locale = this.data[this.locale];
    const out = locale[key];
    return out || this.data[this.defaultLocale][key];
  }
}
