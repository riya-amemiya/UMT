import type { AND } from "$/logic/and";
import type { ArrayReverse } from "$/logic/arrayReverse";
import type { ArrayToUnion } from "$/logic/arrayToUnion";
import type { BGreaterThanA } from "$/logic/bGreaterThanA";
import type { Binary1bitAnd, Binary1bitAndParser } from "$/logic/binary1bitAnd";
import type {
  Binary1bitNand,
  Binary1bitNandParser,
} from "$/logic/binary1bitNand";
import type { Binary1bitNor, Binary1bitNorParser } from "$/logic/binary1bitNor";
import type { Binary1bitor, Binary1bitOrParser } from "$/logic/binary1bitOr";
import type {
  Binary1bitXnor,
  Binary1bitXnorParser,
} from "$/logic/binary1bitXnor";
import type { Binary1bitXOr, Binary1bitXorParser } from "$/logic/binary1bitXor";
import type { BinaryAbs } from "$/logic/binaryAbs";
import type { BinaryAdd, BinaryAddParser } from "$/logic/binaryAdd";
import type { BinaryAnd, BinaryAndParser } from "$/logic/binaryAnd";
import type {
  BinaryComplement,
  BinaryComplementParser,
} from "$/logic/binaryComplement";
import type {
  BinaryFullAdder,
  BinaryFullAdderParser,
} from "$/logic/binaryFullAdder";
import type {
  BinaryHalfAdder,
  BinaryHalfAdderParser,
} from "$/logic/binaryHalfAdder";
import type { BinaryNComplement } from "$/logic/binaryNComplement";
import type { BinaryNand, BinaryNandParser } from "$/logic/binaryNand";
import type { BinaryNor, BinaryNorParser } from "$/logic/binaryNor";
import type { BinaryNot, BinaryNotParser } from "$/logic/binaryNot";
import type { BinaryOr, BinaryOrParser } from "$/logic/binaryOr";
import type {
  BinaryToDecimal,
  BinaryToDecimalParser,
} from "$/logic/binaryToDecimal";
import type { BinaryToHex, BinaryToHexParser } from "$/logic/binaryToHex";
import type { BinaryXnor, BinaryXnorParser } from "$/logic/binaryXnor";
import type { BinaryXor, BinaryXorParser } from "$/logic/binaryXor";
import type { Capitalize } from "$/logic/capitalize";
import type { Decimal1byteTobinary } from "$/logic/decimal1byteTobinary";
import type { Decimal4bitToHex } from "$/logic/decimal4bitToHex";
import type { Decimal4bitTobinary } from "$/logic/decimal4bitTobinary";
import type { DeepPartial } from "$/logic/deepPartial";
import type { DeepRequired } from "$/logic/deepRequired";
import type { Equal } from "$/logic/equal";
import type { First } from "$/logic/first";
import type { First8Chars } from "$/logic/first8Chars";
import type { FirstNChars } from "$/logic/firstNChars";
import type { Hex4bitToDecimal } from "$/logic/hex4bitToDecimal";
import type { HexToBinary, HexToBinaryParser } from "$/logic/hexToBinary";
import type { IF } from "$/logic/if";
import type { IMPLY } from "$/logic/imply";
import type { IsAny } from "$/logic/isAny";
import type { isBoolean } from "$/logic/isBoolean";
import type { IsFloat } from "$/logic/isFloat";
import type { Length } from "$/logic/length";
import type { LengthOfString } from "$/logic/lengthOfString";
import type { NAND } from "$/logic/nand";
import type { NOR } from "$/logic/nor";
import type { NOT } from "$/logic/not";
import type { OR } from "$/logic/or";
import type { Pop } from "$/logic/pop";
import type { PopString } from "$/logic/popString";
import type { Shift } from "$/logic/shift";
import type { ShiftString } from "$/logic/shiftString";
import type { Slice } from "$/logic/slice";
import type { StringReverse } from "$/logic/stringReverse";
import type { StringToArray } from "$/logic/stringToArray";
import type { StringToUnion } from "$/logic/stringToUnion";
import type { ToNumber } from "$/logic/toNumber";
import type { UnionToIntersection } from "$/logic/unionToIntersection";
import type { XNOR } from "$/logic/xnor";
import type { XOR } from "$/logic/xor";
import type { ZeroAorB } from "$/logic/zeroAorB";
import type { ZeroString } from "$/logic/zeroString";

describe("logicType Boolean", () => {
  it("isBoolean", () => {
    const _: [
      isBoolean<0>,
      isBoolean<1>,
      isBoolean<"">,
      isBoolean<"a">,
      isBoolean<true>,
      isBoolean<false>,
      isBoolean<undefined>,
      isBoolean<null>,
      isBoolean<[]>,
      isBoolean<[1]>,
      isBoolean<{ a: 1 }>,
      isBoolean<symbol>,
    ] = [
      false,
      true,
      false,
      true,
      true,
      false,
      false,
      false,
      false,
      true,
      true,
      true,
    ];
    expect(_).toEqual(_);
  });
  it("and", () => {
    const _: [AND<1, "a">, AND<0, true>, AND<true, 0>] = [true, false, false];
    expect(_).toEqual(_);
  });
  it("or", () => {
    const _: [OR<1, 0>, OR<0, 1>, OR<0, 0>] = [true, true, false];
    expect(_).toEqual(_);
  });
  it("not", () => {
    const _: [NOT<0>, NOT<1>] = [true, false];
    expect(_).toEqual(_);
  });
  it("nand", () => {
    const _: [NAND<1, 1>, NAND<1, 0>] = [false, true];
    expect(_).toEqual(_);
  });
  it("nor", () => {
    const _: [NOR<0, 0>, NOR<1, 0>] = [true, false];
    expect(_).toEqual(_);
  });
  it("xor", () => {
    const _: [XOR<1, 1>, XOR<1, 0>, XOR<0, 1>, XOR<0, 0>] = [
      false,
      true,
      true,
      false,
    ];
    expect(_).toEqual(_);
  });
  it("xnor", () => {
    const _: [XNOR<1, 1>, XNOR<1, 0>] = [true, false];
    expect(_).toEqual(_);
  });
  it("imply", () => {
    const _: [IMPLY<0, 0>, IMPLY<1, 0>, IMPLY<1, 1>] = [true, false, true];
    expect(_).toEqual(_);
  });
  it("if", () => {
    const _: [IF<true, 1, 2>, IF<false, 1, 2>] = [1, 2];
    expect(_).toEqual(_);
  });
  it("equal", () => {
    const _: [Equal<1, 1>, Equal<1, 2>] = [true, false];
    expect(_).toEqual(_);
  });
  it("isAny", () => {
    const _: [IsAny<ReturnType<typeof JSON.parse>>, IsAny<unknown>] = [
      true,
      false,
    ];
    expect(_).toEqual(_);
  });
  it("isFloat", () => {
    const _: [IsFloat<1.5>, IsFloat<1>, IsFloat<"1.5">] = [true, false, false];
    expect(_).toEqual(_);
  });
  it("zeroAorB", () => {
    const _: [ZeroAorB<0, 1>, ZeroAorB<1, 0>, ZeroAorB<1, 2>] = [
      true,
      true,
      false,
    ];
    expect(_).toEqual(_);
  });
  it("bGreaterThanA", () => {
    const _: [
      BGreaterThanA<1, 3>,
      BGreaterThanA<3, 1>,
      BGreaterThanA<2, 2>,
      BGreaterThanA<-1, -3>,
      BGreaterThanA<-1, 3>,
      BGreaterThanA<3, -1>,
    ] = [true, false, false, true, true, false];
    expect(_).toEqual(_);
  });
});

describe("logicType Array", () => {
  it("arrayReverse", () => {
    const _: ArrayReverse<[1, 2, 3]> = [3, 2, 1];
    expect(_).toEqual(_);
  });
  it("arrayToUnion", () => {
    const _: ArrayToUnion<[1, "a"]>[] = [1, "a"];
    expect(_).toEqual(_);
  });
  it("first", () => {
    const _: [First<[1, 2]>, [First<[]>] extends [never] ? true : false] = [
      1,
      true,
    ];
    expect(_).toEqual(_);
  });
  it("length", () => {
    const _: Length<[1, 2, 3]> = 3;
    expect(_).toEqual(_);
  });
  it("pop", () => {
    const _: [Pop<[1, 2, 3]>, [Pop<[]>] extends [never] ? true : false] = [
      [1, 2],
      true,
    ];
    expect(_).toEqual(_);
  });
  it("shift", () => {
    const _: [Shift<[1, 2, 3]>, [Shift<[]>] extends [never] ? true : false] = [
      [2, 3],
      true,
    ];
    expect(_).toEqual(_);
  });
});

describe("logicType String", () => {
  it("capitalize", () => {
    const _: [Capitalize<"hello">, Capitalize<"">] = ["Hello", ""];
    expect(_).toEqual(_);
  });
  it("first8Chars", () => {
    const _: [
      First8Chars<"abcdefghij">,
      [First8Chars<"abc">] extends [never] ? true : false,
    ] = ["abcdefgh", true];
    expect(_).toEqual(_);
  });
  it("firstNChars", () => {
    const _: [
      FirstNChars<"hello", 2>,
      [FirstNChars<"he", 3>] extends [never] ? true : false,
    ] = ["he", true];
    expect(_).toEqual(_);
  });
  it("lengthOfString", () => {
    const _: [LengthOfString<"abc">, LengthOfString<"">] = [3, 0];
    expect(_).toEqual(_);
  });
  it("popString", () => {
    const _: [
      PopString<"abc">,
      PopString<"a">,
      [PopString<"">] extends [never] ? true : false,
    ] = ["ab", "", true];
    expect(_).toEqual(_);
  });
  it("shiftString", () => {
    const _: [
      ShiftString<"abc">,
      [ShiftString<"">] extends [never] ? true : false,
    ] = ["bc", true];
    expect(_).toEqual(_);
  });
  it("slice", () => {
    const _: [Slice<"hello", 1, 3>, Slice<"ab", 0, 5>] = ["el", "ab"];
    expect(_).toEqual(_);
  });
  it("stringReverse", () => {
    const _: StringReverse<"abc"> = "cba";
    expect(_).toEqual(_);
  });
  it("stringToArray", () => {
    const _: StringToArray<"abc"> = ["a", "b", "c"];
    expect(_).toEqual(_);
  });
  it("stringToUnion", () => {
    const _: [
      StringToUnion<"abc">[],
      "d" extends StringToUnion<"abc"> ? true : false,
    ] = [["a", "b", "c"], false];
    expect(_).toEqual(_);
  });
  it("toNumber", () => {
    const _: [ToNumber<"42">, [ToNumber<"a">] extends [never] ? true : false] =
      [42, true];
    expect(_).toEqual(_);
  });
  it("zeroString", () => {
    const _: [ZeroString<3>, ZeroString<0>] = ["000", ""];
    expect(_).toEqual(_);
  });
});

describe("logicType Object", () => {
  it("deepPartial", () => {
    const _: [
      DeepPartial<{ a: { b: number }; c: string }>,
      DeepPartial<number>,
    ] = [{ a: {} }, 1];
    expect(_).toEqual(_);
  });
  it("deepRequired", () => {
    const _: [
      DeepRequired<{ a?: { b?: number } }>,
      { a: object } extends DeepRequired<{ a?: { b?: number } }> ? true : false,
      DeepRequired<number>,
    ] = [{ a: { b: 1 } }, false, 1];
    expect(_).toEqual(_);
  });
  it("unionToIntersection", () => {
    const _: UnionToIntersection<{ a: 1 } | { b: 2 }> = { a: 1, b: 2 };
    expect(_).toEqual(_);
  });
});

describe("logicType Binary 1bit", () => {
  it("binary1bitAnd", () => {
    const _: [
      Binary1bitAnd<"1", "1">,
      Binary1bitAnd<"1", "0">,
      Binary1bitAnd<"0", "1">,
      Binary1bitAndParser<"0", "0">,
    ] = ["1", "0", "0", "0"];
    expect(_).toEqual(_);
  });
  it("binary1bitNand", () => {
    const _: [
      Binary1bitNand<"1", "1">,
      Binary1bitNand<"1", "0">,
      Binary1bitNand<"0", "1">,
      Binary1bitNandParser<"0", "0">,
    ] = ["0", "1", "1", "1"];
    expect(_).toEqual(_);
  });
  it("binary1bitNor", () => {
    const _: [
      Binary1bitNor<"1", "1">,
      Binary1bitNor<"1", "0">,
      Binary1bitNor<"0", "1">,
      Binary1bitNorParser<"0", "0">,
    ] = ["0", "0", "0", "1"];
    expect(_).toEqual(_);
  });
  it("binary1bitOr", () => {
    const _: [
      Binary1bitor<"1", "1">,
      Binary1bitor<"1", "0">,
      Binary1bitor<"0", "1">,
      Binary1bitOrParser<"0", "0">,
    ] = ["1", "1", "1", "0"];
    expect(_).toEqual(_);
  });
  it("binary1bitXnor", () => {
    const _: [
      Binary1bitXnor<"1", "1">,
      Binary1bitXnor<"1", "0">,
      Binary1bitXnor<"0", "1">,
      Binary1bitXnorParser<"0", "0">,
    ] = ["1", "0", "0", "1"];
    expect(_).toEqual(_);
  });
  it("binary1bitXor", () => {
    const _: [
      Binary1bitXOr<"1", "1">,
      Binary1bitXOr<"1", "0">,
      Binary1bitXOr<"0", "1">,
      Binary1bitXorParser<"0", "0">,
    ] = ["0", "1", "1", "0"];
    expect(_).toEqual(_);
  });
  it("binaryNot", () => {
    const _: [BinaryNot<"1">, BinaryNot<"0">, BinaryNotParser<"1">] = [
      "0",
      "1",
      "0",
    ];
    expect(_).toEqual(_);
  });
  it("binaryHalfAdder", () => {
    const _: [
      BinaryHalfAdder<"1", "1">,
      BinaryHalfAdder<"1", "0">,
      BinaryHalfAdderParser<"0", "0">,
    ] = ["10", "01", "00"];
    expect(_).toEqual(_);
  });
});

describe("logicType Binary 1byte", () => {
  it("binaryAnd", () => {
    const _: [BinaryAnd<"11001100", "10101010">, BinaryAndParser<"1", "">] = [
      "10001000",
      "",
    ];
    expect(_).toEqual(_);
  });
  it("binaryNand", () => {
    const _: [BinaryNand<"11001100", "10101010">, BinaryNandParser<"1", "">] = [
      "01110111",
      "",
    ];
    expect(_).toEqual(_);
  });
  it("binaryNor", () => {
    const _: [BinaryNor<"11001100", "10101010">, BinaryNorParser<"1", "">] = [
      "00010001",
      "",
    ];
    expect(_).toEqual(_);
  });
  it("binaryOr", () => {
    const _: [BinaryOr<"11001100", "10101010">, BinaryOrParser<"1", "">] = [
      "11101110",
      "",
    ];
    expect(_).toEqual(_);
  });
  it("binaryXnor", () => {
    const _: [BinaryXnor<"11001100", "10101010">, BinaryXnorParser<"1", "">] = [
      "10011001",
      "",
    ];
    expect(_).toEqual(_);
  });
  it("binaryXor", () => {
    const _: [BinaryXor<"11001100", "10101010">, BinaryXorParser<"1", "">] = [
      "01100110",
      "",
    ];
    expect(_).toEqual(_);
  });
  it("binaryAdd", () => {
    const _: [
      BinaryAdd<"00000001", "00000001">,
      BinaryAdd<"00000011", "00000101">,
      BinaryAdd<"11111111", "00000001">,
      BinaryAddParser<"1", "1">,
    ] = ["00000010", "00001000", "00000000", "10"];
    expect(_).toEqual(_);
  });
  it("binaryComplement", () => {
    const _: [
      BinaryComplement<"00000001">,
      BinaryComplement<"11111111">,
      BinaryComplementParser<"1010">,
    ] = ["11111111", "00000001", "0101"];
    expect(_).toEqual(_);
  });
  it("binaryAbs", () => {
    const _: [BinaryAbs<"11111111">, BinaryAbs<"00000101">] = [
      "00000001",
      "00000101",
    ];
    expect(_).toEqual(_);
  });
  it("binaryFullAdder", () => {
    const _: [
      BinaryFullAdder<"0011", "0001">,
      [BinaryFullAdder<"01", "001">] extends [never] ? true : false,
      BinaryFullAdderParser<"1", "1">,
    ] = ["0100", true, "01"];
    expect(_).toEqual(_);
  });
  it("binaryNComplement", () => {
    const _: [BinaryNComplement<"0001">, BinaryNComplement<"0110">] = [
      "1111",
      "1010",
    ];
    expect(_).toEqual(_);
  });
  it("binaryToDecimal", () => {
    const _: [
      BinaryToDecimal<"00000101">,
      BinaryToDecimal<"11111111">,
      BinaryToDecimal<"11111110">,
      BinaryToDecimalParser<"101">,
    ] = [5, "-1", "-2", 5];
    expect(_).toEqual(_);
  });
  it("binaryToHex", () => {
    const _: [
      BinaryToHex<"11111111">,
      BinaryToHex<"00001010">,
      BinaryToHexParser<"1">,
    ] = ["FF", "0A", ""];
    expect(_).toEqual(_);
  });
});

describe("logicType Conversion", () => {
  it("decimal1byteTobinary", () => {
    const _: [
      Decimal1byteTobinary<"0">,
      Decimal1byteTobinary<"10">,
      Decimal1byteTobinary<"127">,
      Decimal1byteTobinary<"-128">,
      Decimal1byteTobinary<"-1">,
      [Decimal1byteTobinary<"128">] extends [never] ? true : false,
    ] = ["00000000", "00001010", "01111111", "10000000", "11111111", true];
    expect(_).toEqual(_);
  });
  it("decimal4bitTobinary", () => {
    const _: [
      Decimal4bitTobinary<"0">,
      Decimal4bitTobinary<"1">,
      Decimal4bitTobinary<"2">,
      Decimal4bitTobinary<"3">,
      Decimal4bitTobinary<"4">,
      Decimal4bitTobinary<"5">,
      Decimal4bitTobinary<"6">,
      Decimal4bitTobinary<"7">,
      Decimal4bitTobinary<"8">,
      Decimal4bitTobinary<"9">,
      Decimal4bitTobinary<"10">,
      Decimal4bitTobinary<"11">,
      Decimal4bitTobinary<"12">,
      Decimal4bitTobinary<"13">,
      Decimal4bitTobinary<"14">,
      Decimal4bitTobinary<"15">,
      [Decimal4bitTobinary<"16">] extends [never] ? true : false,
    ] = [
      "0000",
      "0001",
      "0010",
      "0011",
      "0100",
      "0101",
      "0110",
      "0111",
      "1000",
      "1001",
      "1010",
      "1011",
      "1100",
      "1101",
      "1110",
      "1111",
      true,
    ];
    expect(_).toEqual(_);
  });
  it("decimal4bitToHex", () => {
    const _: [
      Decimal4bitToHex<"9">,
      Decimal4bitToHex<"10">,
      Decimal4bitToHex<"11">,
      Decimal4bitToHex<"12">,
      Decimal4bitToHex<"13">,
      Decimal4bitToHex<"14">,
      Decimal4bitToHex<"15">,
    ] = ["9", "A", "B", "C", "D", "E", "F"];
    expect(_).toEqual(_);
  });
  it("hex4bitToDecimal", () => {
    const _: [
      Hex4bitToDecimal<"9">,
      Hex4bitToDecimal<"A">,
      Hex4bitToDecimal<"B">,
      Hex4bitToDecimal<"C">,
      Hex4bitToDecimal<"D">,
      Hex4bitToDecimal<"E">,
      Hex4bitToDecimal<"F">,
    ] = ["9", "10", "11", "12", "13", "14", "15"];
    expect(_).toEqual(_);
  });
  it("hexToBinary", () => {
    const _: [
      HexToBinary<"FF">,
      HexToBinary<"0A">,
      HexToBinaryParser<"A">,
      HexToBinaryParser<"">,
    ] = ["11111111", "00001010", "1010", ""];
    expect(_).toEqual(_);
  });
});
