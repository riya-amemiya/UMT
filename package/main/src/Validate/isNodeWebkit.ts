import { isBrowser } from "./isBrowser";
import { isNode } from "./isNode";
/**
 * Determines if the current environment is Node-Webkit
 */
export const isNodeWebkit = () => isBrowser() && isNode();
