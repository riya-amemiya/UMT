import { objectUnion } from "./objectUnion";
import { objectMap } from "./objectMap";

export { objectUnion, objectMap };
export class UMTObjectClass {
  private localObjectUnion: typeof objectUnion;
  private localObjectMap: typeof objectMap;
  constructor() {
    this.localObjectUnion = objectUnion;
    this.localObjectMap = objectMap;
  }

  get objectUnion() {
    return this.localObjectUnion;
  }

  get objectMap() {
    return this.localObjectMap;
  }
}

export const UMT_Object = new UMTObjectClass();
