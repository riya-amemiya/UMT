type Types<T> = T extends string
  ? "string"
  : T extends number
  ? "number"
  : T extends boolean
  ? "boolean"
  : T extends Array<infer _>
  ? "array"
  : T extends object
  ? "object"
  : never;

export interface ValidateCoreReturnType<T> {
  validate: boolean;
  message: string;
  type: Types<T>;
}

export interface ValidateReturnType<T> {
  type: Types<T>;
  validate: ValidateFunctionType<T>;
  message?: string;
}

export interface ValidateFunctionType<T> {
  (value: T): boolean;
}
