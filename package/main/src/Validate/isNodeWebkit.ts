import { isBrowser } from "./isBrowser";
import { isNode } from "./isNode";
/**
 * node-webkit環境かどうかを判定します。
 */
export const isNodeWebkit = isBrowser && isNode;
