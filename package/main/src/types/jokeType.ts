import type { Add } from "$/math/add";
import type { Subtract } from "$/math/subtract";
import type { Multiply } from "$/mathType";

export type ThreeStepsForwardTwoStepsBack<T extends number> = T extends 1
  ? 1
  : T extends 2
    ? 2
    : T extends 3
      ? 3
      : Add<3, Multiply<3, Subtract<T, 3>>>;
