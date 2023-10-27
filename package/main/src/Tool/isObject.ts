export const isObject = <T extends { [key: string]: unknown }>(
  object: unknown,
): object is T => {
  return (
    typeof object === "object" && object !== null && !Array.isArray(object)
  );
};
