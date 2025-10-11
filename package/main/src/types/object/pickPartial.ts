/**
 * Makes specified keys optional in an object type
 * @template T - The object type
 * @template K - The keys to make optional
 */
export type PickPartial<T, K extends keyof T> = Omit<T, K> &
  Partial<Pick<T, K>>;
