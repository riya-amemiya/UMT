import { Add, Multiply, Subtract } from "./mathType";

export type ThreeStepsForwardTwoStepsBack<T extends number> = T extends 1
  ? 1
  : T extends 2
    ? 2
    : T extends 3
      ? 3
      : Add<3, Multiply<3, Subtract<T, 3>>>;
