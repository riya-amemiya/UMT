export type ConvertMonTypeNoZero<T extends string> = T extends `${0}${infer U}`
  ? U
  : T;
