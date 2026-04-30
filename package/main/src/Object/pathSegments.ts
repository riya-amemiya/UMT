export const pathSegments = (path: string | string[]): string[] =>
  typeof path === "string" ? path.split(".") : path;
