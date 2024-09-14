import type { Int } from "./int";
import type { IntWithoutZero } from "./intWithoutZero";

export type FourDigit = `${IntWithoutZero}${Int}${Int}${Int}`;
