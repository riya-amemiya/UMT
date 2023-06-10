import { arraysJoin } from "./arraysJoin";
import { getArraysCommon } from "./getArraysCommon";
import { getArraysDiff } from "./getArraysDiff";
import { quickSort } from "./quickSort";
import { sum } from "./sum";
import { arrayMap } from "./arrayMap";

<<<<<<< HEAD
export {
  arraysJoin,
  getArraysCommon,
  getArraysDiff,
  quickSort,
  sum,
  arrayMap,
  ArrayWrapper,
};
=======
export { arraysJoin, getArraysCommon, getArraysDiff, quickSort, sum, arrayMap };

>>>>>>> origin/main
export class UMTArrayClass {
  private localArraysJoin: typeof arraysJoin;
  private localGetArraysCommon: typeof getArraysCommon;
  private localGetArraysDiff: typeof getArraysDiff;
  private localQuickSort: typeof quickSort;
  private locaLsum: typeof sum;
  private localArrayMap: typeof arrayMap;
  constructor() {
    this.localArraysJoin = arraysJoin;
    this.localGetArraysCommon = getArraysCommon;
    this.localGetArraysDiff = getArraysDiff;
    this.localQuickSort = quickSort;
    this.locaLsum = sum;
    this.localArrayMap = arrayMap;
  }

  get arraysJoin() {
    return this.localArraysJoin;
  }
  get getArraysCommon() {
    return this.localGetArraysCommon;
  }
  get getArraysDiff() {
    return this.localGetArraysDiff;
  }
  get quickSort() {
    return this.localQuickSort;
  }
  get sum() {
    return this.locaLsum;
  }
  get arrayMap() {
    return this.localArrayMap;
  }
}

export const UMT_Array = new UMTArrayClass();
