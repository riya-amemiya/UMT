import type { Add } from "$/math/add";
import type { Multiply } from "$/math/multiply";
import type { Subtract } from "$/math/subtract";

export type ThreeStepsForwardTwoStepsBack<T extends number> = T extends 1
  ? 1
  : T extends 2
    ? 2
    : T extends 3
      ? 3
      : Add<3, Multiply<3, Subtract<T, 3>>>;
