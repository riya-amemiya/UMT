export type PickDeepKey<T extends object> = keyof T extends infer K
  ? K extends keyof T
    ? T[K] extends object
      ? K extends string
        ? PickDeepKey<T[K]> extends string
          ? K | `${K}.${PickDeepKey<T[K]>}`
          : K
        : never
      : K
    : never
  : never;
