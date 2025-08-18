// biome-ignore lint/correctness/noNodejsModules: ignore
import process from "node:process";
/**
 * Determines if the current environment is Node.js
 */
export const isNode = () => {
  try {
    return process !== undefined && typeof require !== "undefined";
  } catch {
    return false;
  }
};
