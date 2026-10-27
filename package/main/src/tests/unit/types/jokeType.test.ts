import type { ThreeStepsForwardTwoStepsBack } from "$/joke/threeStepsForwardTwoStepsBack";

describe("jokeType", () => {
  it("threeStepsForwardTwoStepsBack", () => {
    const _: [
      ThreeStepsForwardTwoStepsBack<1>,
      ThreeStepsForwardTwoStepsBack<2>,
      ThreeStepsForwardTwoStepsBack<3>,
      ThreeStepsForwardTwoStepsBack<4>,
      ThreeStepsForwardTwoStepsBack<5>,
    ] = [1, 2, 3, 6, 9];
    expect(_).toEqual(_);
  });
});
