export type UMT_i18nData<
  T extends { [key: string]: string },
  L extends string,
> = {
  [key in L]: T;
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
  constructor(data: T, locale: keyof T) {
    this.data = data;
    this.locale = locale;
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

  public translate(key: keyof T[keyof T]) {
    return this.data[this.locale][key];
  }
}
