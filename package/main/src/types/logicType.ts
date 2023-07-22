// booleanに変換する型
export type isBoolean<X> = X extends number
  ? X extends 0
    ? false
    : true
  : X extends string
  ? X extends ""
    ? false
    : true
  : X extends boolean
  ? X
  : X extends undefined
  ? false
  : X extends null
  ? false
  : X extends object
  ? X extends unknown[]
    ? X extends []
      ? false
      : true
    : true
  : X extends (...args: unknown[]) => unknown
  ? false
  : true;

export type AND<X, Y> = isBoolean<X> extends true
  ? isBoolean<Y> extends true
    ? true
    : false
  : false;

export type OR<X, Y> = isBoolean<X> extends true
  ? true
  : isBoolean<Y> extends true
  ? true
  : false;

export type XOR<X, Y> = isBoolean<X> extends true
  ? isBoolean<Y> extends true
    ? false
    : true
  : isBoolean<Y> extends true
  ? true
  : false;
export type NOT<X> = isBoolean<X> extends true ? false : true;
export type NAND<X, Y> = NOT<AND<X, Y>>;
export type NOR<X, Y> = NOT<OR<X, Y>>;
export type XNOR<X, Y> = NOT<XOR<X, Y>>;
export type IMPLY<X, Y> = NOT<X> extends true ? true : isBoolean<Y>;

// 文字列を配列に変換する型
export type StringToArray<
  S extends string,
  T extends unknown[] = [],
> = S extends `${infer F}${infer R}` ? StringToArray<R, [...T, F]> : T;

// 配列の長さを取得する型
export type Length<T extends unknown[]> = T["length"];

// 文字を配列に変換して長さを取得する型
export type LengthOfString<S extends string,> = StringToArray<S>["length"];

// 配列の先頭を削除する型
export type Shift<T extends unknown[]> = T extends [unknown, ...infer R]
  ? R
  : never;

// 配列の末尾を削除する型
export type Pop<T extends unknown[]> = T extends [...infer R, unknown]
  ? R
  : never;

// 文字列の先頭を削除する型
export type ShiftString<S extends string> = S extends `${string}${infer R}`
  ? R
  : never;

// 文字列の末尾を削除する型
export type PopString<S extends string> = S extends `${infer R}${string}`
  ? R
  : never;

// 文字列をUnion型に変換する型
export type StringToUnion<S extends string> = S extends `${infer F}${infer R}`
  ? F | StringToUnion<R>
  : never;

// any型かどうかを判定する型
export type IsAny<T> = 0 extends 1 & T ? true : false;

// 文字列を反転する型
export type StringReverse<S extends string> = S extends `${infer F}${infer R}`
  ? `${StringReverse<R>}${F}`
  : "";

// 先頭から8文字を取得する型
export type First8Chars<
  S extends string,
  T extends unknown[] = [],
> = T["length"] extends 8
  ? T extends [
      infer C1,
      infer C2,
      infer C3,
      infer C4,
      infer C5,
      infer C6,
      infer C7,
      infer C8,
      ...unknown[],
    ]
    ? `${C1 & string}${C2 & string}${C3 & string}${C4 & string}${C5 &
        string}${C6 & string}${C7 & string}${C8 & string}`
    : never
  : S extends `${infer Head}${infer Rest}`
  ? First8Chars<Rest, [...T, Head]>
  : never;


export type FirstNChars<
  S extends string,
  N extends number,
  C extends unknown[] = [],
  A extends string = "",
> = C["length"] extends N
  ? A
  : S extends `${infer Head}${infer Rest}`
  ? FirstNChars<Rest, N, [...C, Head], `${A}${Head}`>
  : never;

export type ZeroString<
  N extends number,
  A extends string = "",
> = LengthOfString<A> extends N ? A : ZeroString<N, `${A}0`>;

// 半加算器
export type binaryHalfAdder<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = binaryHalfAdderParser<StringReverse<X>, StringReverse<Y>>;

type binaryHalfAdderParser<
  X extends string,
  Y extends string,
  C extends string = "",
> = C extends ""
  ? binaryHalfAdderParser<X, Y, binary1bitANDParser<X, Y>>
  : `${C}${binary1bitANDParser<binaryNOTParser<C>, binary1bitORParser<X, Y>>}`;

// 全加算器
export type binaryFullAdder<
  X extends string,
  Y extends string,
  B extends number = 8,
> = LengthOfString<X> extends LengthOfString<Y>
  ? FirstNChars<
      ShiftString<
        StringReverse<
          binaryFullAdderParser<
            StringReverse<FirstNChars<X, B>>,
            StringReverse<FirstNChars<Y, B>>
          >
        >
      >,
      B
    >
  : never;

// 任意のバイト数の加算器
type binaryFullAdderParser<
  X extends string,
  Y extends string,
  A extends string = "",
  C extends string = "0",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? binaryHalfAdderParser<F, F2> extends `${infer F3}${infer R3}`
      ? binaryHalfAdderParser<R3, C> extends `${infer F4}${infer R4}`
        ? binaryFullAdderParser<R, R2, `${A}${R4}`, binary1bitORParser<F3, F4>>
        : never
      : never
    : never
  : `${A}${C}`;

// 1バイトまでの加算器
type binaryAddParser<
  A extends string,
  B extends string,
  C extends "0" | "1" = "0",
> = [A, B] extends [`${infer A1}${infer A2}`, `${infer B1}${infer B2}`]
  ? A1 extends "0"
    ? B1 extends "0"
      ? C extends "0"
        ? `${binaryAddParser<A2, B2, "0">}0`
        : `${binaryAddParser<A2, B2, "0">}1`
      : C extends "0"
      ? `${binaryAddParser<A2, B2, "0">}1`
      : `${binaryAddParser<A2, B2, "1">}0`
    : B1 extends "0"
    ? C extends "0"
      ? `${binaryAddParser<A2, B2, "0">}1`
      : `${binaryAddParser<A2, B2, "1">}0`
    : C extends "0"
    ? `${binaryAddParser<A2, B2, "1">}0`
    : `${binaryAddParser<A2, B2, "1">}1`
  : `${C}`;

export type binaryAdd<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = StringReverse<
  First8Chars<
    StringReverse<binaryAddParser<StringReverse<X>, StringReverse<Y>>>
  >
>;

type binaryComplementParser<X extends string> = X extends `${infer L}${infer R}`
  ? L extends "0"
    ? `1${binaryComplementParser<R>}`
    : `0${binaryComplementParser<R>}`
  : "";

// 1バイトの2の補数を求める型
export type binaryComplement<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = First8Chars<binaryAdd<binaryComplementParser<X>, "00000001">>;

// 2の補数を求める型
export type binaryNComplement<X extends string,> = binaryFullAdder<
  binaryComplementParser<X>,
  StringReverse<`1${ShiftString<ZeroString<LengthOfString<X>>>}`>
>;

type binaryToDecimalParser<
  X extends string,
  C extends unknown[] = [""],
  A extends unknown[] = [""],
  FL extends boolean = false,
> = X extends `${infer F}${infer R}`
  ? LengthOfString<X> extends 8
    ? F extends "1"
      ? `-${binaryToDecimalParser<StringReverse<R>, C, A, true>}`
      : binaryToDecimalParser<StringReverse<R>, C, A, FL>
    : FL extends false
    ? F extends "1"
      ? binaryToDecimalParser<R, [...C, ...C], [...A, ...C], FL>
      : binaryToDecimalParser<R, [...C, ...C], A, FL>
    : F extends "1"
    ? binaryToDecimalParser<R, [...C, ...C], A, FL>
    : binaryToDecimalParser<R, [...C, ...C], [...A, ...C], FL>
  : FL extends true
  ? Length<A>
  : Length<Shift<A>>;

// 2進数を10進数に変換する型
export type binaryToDecimal<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = binaryToDecimalParser<X>;

// 2進数の絶対値を求める型
export type binaryAbs<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = X extends `${infer F}${infer _}`
  ? F extends "1"
    ? binaryComplement<X>
    : X
  : never;

// 4bitの10進数を16進数に変換する型
export type decimal4bitToHex<X extends string> = X extends "10"
  ? "A"
  : X extends "11"
  ? "B"
  : X extends "12"
  ? "C"
  : X extends "13"
  ? "D"
  : X extends "14"
  ? "E"
  : X extends "15"
  ? "F"
  : X;

// 4bitの16進数を10進数に変換する型
export type hex4bitToDecimal<X extends string> = X extends "A"
  ? "10"
  : X extends "B"
  ? "11"
  : X extends "C"
  ? "12"
  : X extends "D"
  ? "13"
  : X extends "E"
  ? "14"
  : X extends "F"
  ? "15"
  : X;

// 4bitの10進数を2進数に変換する型
export type decimal4bitTobinary<X extends string> = X extends "0"
  ? "0000"
  : X extends "1"
  ? "0001"
  : X extends "2"
  ? "0010"
  : X extends "3"
  ? "0011"
  : X extends "4"
  ? "0100"
  : X extends "5"
  ? "0101"
  : X extends "6"
  ? "0110"
  : X extends "7"
  ? "0111"
  : X extends "8"
  ? "1000"
  : X extends "9"
  ? "1001"
  : X extends "10"
  ? "1010"
  : X extends "11"
  ? "1011"
  : X extends "12"
  ? "1100"
  : X extends "13"
  ? "1101"
  : X extends "14"
  ? "1110"
  : X extends "15"
  ? "1111"
  : never;

// 1bitの2進数を10進数に変換する型
export type decimal1biteTobinary<X extends string> = X extends "0"
  ? "00000000"
  : X extends "1"
  ? "00000001"
  : X extends "2"
  ? "00000010"
  : X extends "3"
  ? "00000011"
  : X extends "4"
  ? "00000100"
  : X extends "5"
  ? "00000101"
  : X extends "6"
  ? "00000110"
  : X extends "7"
  ? "00000111"
  : X extends "8"
  ? "00001000"
  : X extends "9"
  ? "00001001"
  : X extends "10"
  ? "00001010"
  : X extends "11"
  ? "00001011"
  : X extends "12"
  ? "00001100"
  : X extends "13"
  ? "00001101"
  : X extends "14"
  ? "00001110"
  : X extends "15"
  ? "00001111"
  : X extends "16"
  ? "00010000"
  : X extends "17"
  ? "00010001"
  : X extends "18"
  ? "00010010"
  : X extends "19"
  ? "00010011"
  : X extends "20"
  ? "00010100"
  : X extends "21"
  ? "00010101"
  : X extends "22"
  ? "00010110"
  : X extends "23"
  ? "00010111"
  : X extends "24"
  ? "00011000"
  : X extends "25"
  ? "00011001"
  : X extends "26"
  ? "00011010"
  : X extends "27"
  ? "00011011"
  : X extends "28"
  ? "00011100"
  : X extends "29"
  ? "00011101"
  : X extends "30"
  ? "00011110"
  : X extends "31"
  ? "00011111"
  : X extends "32"
  ? "00100000"
  : X extends "33"
  ? "00100001"
  : X extends "34"
  ? "00100010"
  : X extends "35"
  ? "00100011"
  : X extends "36"
  ? "00100100"
  : X extends "37"
  ? "00100101"
  : X extends "38"
  ? "00100110"
  : X extends "39"
  ? "00100111"
  : X extends "40"
  ? "00101000"
  : X extends "41"
  ? "00101001"
  : X extends "42"
  ? "00101010"
  : X extends "43"
  ? "00101011"
  : X extends "44"
  ? "00101100"
  : X extends "45"
  ? "00101101"
  : X extends "46"
  ? "00101110"
  : X extends "47"
  ? "00101111"
  : X extends "48"
  ? "00110000"
  : X extends "49"
  ? "00110001"
  : X extends "50"
  ? "00110010"
  : X extends "51"
  ? "00110011"
  : X extends "52"
  ? "00110100"
  : X extends "53"
  ? "00110101"
  : X extends "54"
  ? "00110110"
  : X extends "55"
  ? "00110111"
  : X extends "56"
  ? "00111000"
  : X extends "57"
  ? "00111001"
  : X extends "58"
  ? "00111010"
  : X extends "59"
  ? "00111011"
  : X extends "60"
  ? "00111100"
  : X extends "61"
  ? "00111101"
  : X extends "62"
  ? "00111110"
  : X extends "63"
  ? "00111111"
  : X extends "64"
  ? "01000000"
  : X extends "65"
  ? "01000001"
  : X extends "66"
  ? "01000010"
  : X extends "67"
  ? "01000011"
  : X extends "68"
  ? "01000100"
  : X extends "69"
  ? "01000101"
  : X extends "70"
  ? "01000110"
  : X extends "71"
  ? "01000111"
  : X extends "72"
  ? "01001000"
  : X extends "73"
  ? "01001001"
  : X extends "74"
  ? "01001010"
  : X extends "75"
  ? "01001011"
  : X extends "76"
  ? "01001100"
  : X extends "77"
  ? "01001101"
  : X extends "78"
  ? "01001110"
  : X extends "79"
  ? "01001111"
  : X extends "80"
  ? "01010000"
  : X extends "81"
  ? "01010001"
  : X extends "82"
  ? "01010010"
  : X extends "83"
  ? "01010011"
  : X extends "84"
  ? "01010100"
  : X extends "85"
  ? "01010101"
  : X extends "86"
  ? "01010110"
  : X extends "87"
  ? "01010111"
  : X extends "88"
  ? "01011000"
  : X extends "89"
  ? "01011001"
  : X extends "90"
  ? "01011010"
  : X extends "91"
  ? "01011011"
  : X extends "92"
  ? "01011100"
  : X extends "93"
  ? "01011101"
  : X extends "94"
  ? "01011110"
  : X extends "95"
  ? "01011111"
  : X extends "96"
  ? "01100000"
  : X extends "97"
  ? "01100001"
  : X extends "98"
  ? "01100010"
  : X extends "99"
  ? "01100011"
  : X extends "100"
  ? "01100100"
  : X extends "101"
  ? "01100101"
  : X extends "102"
  ? "01100110"
  : X extends "103"
  ? "01100111"
  : X extends "104"
  ? "01101000"
  : X extends "105"
  ? "01101001"
  : X extends "106"
  ? "01101010"
  : X extends "107"
  ? "01101011"
  : X extends "108"
  ? "01101100"
  : X extends "109"
  ? "01101101"
  : X extends "110"
  ? "01101110"
  : X extends "111"
  ? "01101111"
  : X extends "112"
  ? "01110000"
  : X extends "113"
  ? "01110001"
  : X extends "114"
  ? "01110010"
  : X extends "115"
  ? "01110011"
  : X extends "116"
  ? "01110100"
  : X extends "117"
  ? "01110101"
  : X extends "118"
  ? "01110110"
  : X extends "119"
  ? "01110111"
  : X extends "120"
  ? "01111000"
  : X extends "121"
  ? "01111001"
  : X extends "122"
  ? "01111010"
  : X extends "123"
  ? "01111011"
  : X extends "124"
  ? "01111100"
  : X extends "125"
  ? "01111101"
  : X extends "126"
  ? "01111110"
  : X extends "127"
  ? "01111111"
  : X extends "-128"
  ? "10000000"
  : X extends "-127"
  ? "10000001"
  : X extends "-126"
  ? "10000010"
  : X extends "-125"
  ? "10000011"
  : X extends "-124"
  ? "10000100"
  : X extends "-123"
  ? "10000101"
  : X extends "-122"
  ? "10000110"
  : X extends "-121"
  ? "10000111"
  : X extends "-120"
  ? "10001000"
  : X extends "-119"
  ? "10001001"
  : X extends "-118"
  ? "10001010"
  : X extends "-117"
  ? "10001011"
  : X extends "-116"
  ? "10001100"
  : X extends "-115"
  ? "10001101"
  : X extends "-114"
  ? "10001110"
  : X extends "-113"
  ? "10001111"
  : X extends "-112"
  ? "10010000"
  : X extends "-111"
  ? "10010001"
  : X extends "-110"
  ? "10010010"
  : X extends "-109"
  ? "10010011"
  : X extends "-108"
  ? "10010100"
  : X extends "-107"
  ? "10010101"
  : X extends "-106"
  ? "10010110"
  : X extends "-105"
  ? "10010111"
  : X extends "-104"
  ? "10011000"
  : X extends "-103"
  ? "10011001"
  : X extends "-102"
  ? "10011010"
  : X extends "-101"
  ? "10011011"
  : X extends "-100"
  ? "10011100"
  : X extends "-99"
  ? "10011101"
  : X extends "-98"
  ? "10011110"
  : X extends "-97"
  ? "10011111"
  : X extends "-96"
  ? "10100000"
  : X extends "-95"
  ? "10100001"
  : X extends "-94"
  ? "10100010"
  : X extends "-93"
  ? "10100011"
  : X extends "-92"
  ? "10100100"
  : X extends "-91"
  ? "10100101"
  : X extends "-90"
  ? "10100110"
  : X extends "-89"
  ? "10100111"
  : X extends "-88"
  ? "10101000"
  : X extends "-87"
  ? "10101001"
  : X extends "-86"
  ? "10101010"
  : X extends "-85"
  ? "10101011"
  : X extends "-84"
  ? "10101100"
  : X extends "-83"
  ? "10101101"
  : X extends "-82"
  ? "10101110"
  : X extends "-81"
  ? "10101111"
  : X extends "-80"
  ? "10110000"
  : X extends "-79"
  ? "10110001"
  : X extends "-78"
  ? "10110010"
  : X extends "-77"
  ? "10110011"
  : X extends "-76"
  ? "10110100"
  : X extends "-75"
  ? "10110101"
  : X extends "-74"
  ? "10110110"
  : X extends "-73"
  ? "10110111"
  : X extends "-72"
  ? "10111000"
  : X extends "-71"
  ? "10111001"
  : X extends "-70"
  ? "10111010"
  : X extends "-69"
  ? "10111011"
  : X extends "-68"
  ? "10111100"
  : X extends "-67"
  ? "10111101"
  : X extends "-66"
  ? "10111110"
  : X extends "-65"
  ? "10111111"
  : X extends "-64"
  ? "11000000"
  : X extends "-63"
  ? "11000001"
  : X extends "-62"
  ? "11000010"
  : X extends "-61"
  ? "11000011"
  : X extends "-60"
  ? "11000100"
  : X extends "-59"
  ? "11000101"
  : X extends "-58"
  ? "11000110"
  : X extends "-57"
  ? "11000111"
  : X extends "-56"
  ? "11001000"
  : X extends "-55"
  ? "11001001"
  : X extends "-54"
  ? "11001010"
  : X extends "-53"
  ? "11001011"
  : X extends "-52"
  ? "11001100"
  : X extends "-51"
  ? "11001101"
  : X extends "-50"
  ? "11001110"
  : X extends "-49"
  ? "11001111"
  : X extends "-48"
  ? "11010000"
  : X extends "-47"
  ? "11010001"
  : X extends "-46"
  ? "11010010"
  : X extends "-45"
  ? "11010011"
  : X extends "-44"
  ? "11010100"
  : X extends "-43"
  ? "11010101"
  : X extends "-42"
  ? "11010110"
  : X extends "-41"
  ? "11010111"
  : X extends "-40"
  ? "11011000"
  : X extends "-39"
  ? "11011001"
  : X extends "-38"
  ? "11011010"
  : X extends "-37"
  ? "11011011"
  : X extends "-36"
  ? "11011100"
  : X extends "-35"
  ? "11011101"
  : X extends "-34"
  ? "11011110"
  : X extends "-33"
  ? "11011111"
  : X extends "-32"
  ? "11100000"
  : X extends "-31"
  ? "11100001"
  : X extends "-30"
  ? "11100010"
  : X extends "-29"
  ? "11100011"
  : X extends "-28"
  ? "11100100"
  : X extends "-27"
  ? "11100101"
  : X extends "-26"
  ? "11100110"
  : X extends "-25"
  ? "11100111"
  : X extends "-24"
  ? "11101000"
  : X extends "-23"
  ? "11101001"
  : X extends "-22"
  ? "11101010"
  : X extends "-21"
  ? "11101011"
  : X extends "-20"
  ? "11101100"
  : X extends "-19"
  ? "11101101"
  : X extends "-18"
  ? "11101110"
  : X extends "-17"
  ? "11101111"
  : X extends "-16"
  ? "11110000"
  : X extends "-15"
  ? "11110001"
  : X extends "-14"
  ? "11110010"
  : X extends "-13"
  ? "11110011"
  : X extends "-12"
  ? "11110100"
  : X extends "-11"
  ? "11110101"
  : X extends "-10"
  ? "11110110"
  : X extends "-9"
  ? "11110111"
  : X extends "-8"
  ? "11111000"
  : X extends "-7"
  ? "11111001"
  : X extends "-6"
  ? "11111010"
  : X extends "-5"
  ? "11111011"
  : X extends "-4"
  ? "11111100"
  : X extends "-3"
  ? "11111101"
  : X extends "-2"
  ? "11111110"
  : X extends "-1"
  ? "11111111"
  : never;

type binaryToHexParser<
  X extends string,
  C extends string = "",
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? LengthOfString<`${F}${C}`> extends 4
    ? binaryToHexParser<
        R,
        "",
        `${A}${decimal4bitToHex<`${binaryToDecimalParser<`${F}${C}`>}`>}`
      >
    : binaryToHexParser<R, `${F}${C}`, A>
  : A;

// 1バイトの2進数を16進数に変換する型
export type binaryToHex<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = binaryToHexParser<X>;

type hexTobinaryParser<
  X extends string,
  C extends string = "",
> = LengthOfString<X> extends 1
  ? `${C}${decimal4bitTobinary<hex4bitToDecimal<X>>}`
  : X extends `${infer F}${infer R}`
  ? hexTobinaryParser<R, `${C}${decimal4bitTobinary<hex4bitToDecimal<F>>}`>
  : C;

// 16進数を2進数に変換する型
export type hexTobinary<
  X extends `${
    | 0
    | 1
    | 2
    | 3
    | 4
    | 5
    | 6
    | 7
    | 8
    | 9
    | "A"
    | "B"
    | "C"
    | "D"
    | "E"
    | "F"}${
    | 0
    | 1
    | 2
    | 3
    | 4
    | 5
    | 6
    | 7
    | 8
    | 9
    | "A"
    | "B"
    | "C"
    | "D"
    | "E"
    | "F"}`,
> = hexTobinaryParser<X>;

// 2進数のANDを求める型
export type binaryAND<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = binaryANDParser<X, Y>;

type binaryANDParser<
  X extends string,
  Y extends string,
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? F extends "1"
      ? F2 extends "1"
        ? binaryANDParser<R, R2, `${A}1`>
        : binaryANDParser<R, R2, `${A}0`>
      : binaryANDParser<R, R2, `${A}0`>
    : A
  : A;

// 1bitの2進数のANDを求める型
export type binary1bitAND<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = binary1bitANDParser<X, Y>;

type binary1bitANDParser<X extends string, Y extends string> = X extends "1"
  ? Y extends "1"
    ? "1"
    : "0"
  : "0";

// 2進数のORを求める型
export type binaryOR<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = binaryORParser<X, Y>;

type binaryORParser<
  X extends string,
  Y extends string,
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? F extends "1"
      ? binaryORParser<R, R2, `${A}1`>
      : F2 extends "1"
      ? binaryORParser<R, R2, `${A}1`>
      : binaryORParser<R, R2, `${A}0`>
    : A
  : A;

// 1bitの2進数のORを求める型
export type binary1bitOR<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = binary1bitORParser<X, Y>;

type binary1bitORParser<X extends string, Y extends string> = X extends "1"
  ? "1"
  : Y extends "1"
  ? "1"
  : "0";

// 2進数のXORを求める型
export type binaryXOR<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = binaryXORParser<X, Y>;

type binaryXORParser<
  X extends string,
  Y extends string,
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? F extends "1"
      ? F2 extends "1"
        ? binaryXORParser<R, R2, `${A}0`>
        : binaryXORParser<R, R2, `${A}1`>
      : F2 extends "1"
      ? binaryXORParser<R, R2, `${A}1`>
      : binaryXORParser<R, R2, `${A}0`>
    : A
  : A;

// 1bitの2進数のXORを求める型
export type binary1bitXOR<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = binary1bitXORParser<X, Y>;

type binary1bitXORParser<X extends string, Y extends string> = X extends "1"
  ? Y extends "1"
    ? "0"
    : "1"
  : Y extends "1"
  ? "1"
  : "0";

// 2進数のNot ANDを求める型
export type binaryNAND<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = binaryNANDParser<X, Y>;

type binaryNANDParser<
  X extends string,
  Y extends string,
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? F extends "1"
      ? F2 extends "1"
        ? binaryNANDParser<R, R2, `${A}0`>
        : binaryNANDParser<R, R2, `${A}1`>
      : binaryNANDParser<R, R2, `${A}1`>
    : A
  : A;

// 1bitの2進数のNot ANDを求める型
export type binary1bitNAND<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = binary1bitNANDParser<X, Y>;

type binary1bitNANDParser<X extends string, Y extends string> = X extends "1"
  ? Y extends "1"
    ? "0"
    : "1"
  : "1";

// 2進数のNot ORを求める型
export type binaryNOR<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = binaryNORParser<X, Y>;

type binaryNORParser<
  X extends string,
  Y extends string,
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? F extends "1"
      ? binaryNORParser<R, R2, `${A}0`>
      : F2 extends "1"
      ? binaryNORParser<R, R2, `${A}0`>
      : binaryNORParser<R, R2, `${A}1`>
    : A
  : A;

// 1bitの2進数のNot ORを求める型
export type binary1bitNOR<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = binary1bitNORParser<X, Y>;

type binary1bitNORParser<X extends string, Y extends string> = X extends "1"
  ? "0"
  : Y extends "1"
  ? "0"
  : "1";

// 2進数のNot XORを求める型
export type binaryXNOR<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = binaryXNORParser<X, Y>;

type binaryXNORParser<
  X extends string,
  Y extends string,
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? F extends "1"
      ? F2 extends "1"
        ? binaryXNORParser<R, R2, `${A}1`>
        : binaryXNORParser<R, R2, `${A}0`>
      : F2 extends "1"
      ? binaryXNORParser<R, R2, `${A}0`>
      : binaryXNORParser<R, R2, `${A}1`>
    : A
  : A;

// 1bitの2進数のNot XORを求める型
export type binary1bitXNOR<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = binary1bitXNORParser<X, Y>;

type binary1bitXNORParser<X extends string, Y extends string> = X extends "1"
  ? Y extends "1"
    ? "1"
    : "0"
  : Y extends "1"
  ? "0"
  : "1";

// 2進数のNOTを求める型
export type binaryNOT<X extends `${0 | 1}`> = binaryNOTParser<X>;

type binaryNOTParser<X extends string,> = X extends "1" ? "0" : "1";
