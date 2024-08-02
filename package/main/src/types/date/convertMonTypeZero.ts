export type ConvertMonTypeZero<T extends string | number> =
  `${T}` extends `${1}${0 | 1 | 2}` ? T : `${0}${T}`;
