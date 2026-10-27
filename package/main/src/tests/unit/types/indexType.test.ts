import type {
  Add,
  ChunkArrayType,
  FormatValue,
  GetEnumValues,
  HoursAm,
  IF,
  Int,
  MonTypeInt,
  PickPartial,
  SimplifiedUserAgentInfoOs,
  ThreeStepsForwardTwoStepsBack,
  TimeUnit,
} from "$/index";
import type { FormatData } from "$/string/formatString/formatString";

describe("types index", () => {
  it("re-exports every module", () => {
    const _: [
      ChunkArrayType<[1, 2], 1>,
      HoursAm,
      MonTypeInt,
      GetEnumValues<{ a: 1 }>,
      Int,
      ThreeStepsForwardTwoStepsBack<4>,
      IF<true, 1, 2>,
      Add<1, 1>,
      PickPartial<{ a: 1 }, "a">,
      FormatValue,
      FormatData,
      TimeUnit,
      SimplifiedUserAgentInfoOs,
    ] = [[[1], [2]], "00", 1, 1, 0, 6, 1, 2, {}, "a", ["a"], "hours", "ios"];
    expect(_).toEqual(_);
  });
});
