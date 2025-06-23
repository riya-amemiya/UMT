import type { BinaryComplementParser } from "./binaryComplement";
import type { BinaryFullAdder } from "./binaryFullAdder";
import type { LengthOfString } from "./lengthOfString";
import type { ShiftString } from "./shiftString";
import type { StringReverse } from "./stringReverse";
import type { ZeroString } from "./zeroString";

// Type to calculate two's complement
export type BinaryNComplement<X extends string> = BinaryFullAdder<
  BinaryComplementParser<X>,
  StringReverse<`1${ShiftString<ZeroString<LengthOfString<X>>>}`>,
  LengthOfString<X>
>;
