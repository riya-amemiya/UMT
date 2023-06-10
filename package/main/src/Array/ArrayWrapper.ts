import { arrayMap } from "./arrayMap";

export class ArrayWrapper {
  private readonly localArrayMap: typeof arrayMap;
  // rome-ignore lint/suspicious/noExplicitAny: <explanation>
  private localData: any[] = [];
  constructor() {
    this.localArrayMap = arrayMap;
    this.localData = [];
  }

  map<T, U>(
    array: T[],
    callbackfn: (value: T, index: number, rowArray: typeof array) => U,
  ) {
    this.localData = this.localArrayMap(array, callbackfn);
    return this;
  }

  getData() {
    return this.localData;
  }
}
