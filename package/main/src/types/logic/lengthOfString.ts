import type { StringToArray } from "./stringToArray";

// 文字を配列に変換して長さを取得する型
export type LengthOfString<S extends string> = StringToArray<S>["length"];
