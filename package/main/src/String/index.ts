import { reverseString } from "./reverseString";

export { reverseString };
export class UMTStringClass {
  private localReverseString: typeof reverseString;
  constructor() {
    this.localReverseString = reverseString;
  }
  get reverseString() {
    return this.localReverseString;
  }
}
export const UMT_String = new UMTStringClass();
