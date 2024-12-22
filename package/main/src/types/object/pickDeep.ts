export type PickDeep<T extends object> = keyof T extends infer K
  ? K extends keyof T
    ? T[K] extends object
      ? PickDeep<T[K]>
      : T[K]
    : never
  : never;
