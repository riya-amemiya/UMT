import type { ThreeStepsForwardTwoStepsBack } from "$/joke/index";

describe("joke index", () => {
  it("ThreeStepsForwardTwoStepsBack<1>", () => {
    const _: ThreeStepsForwardTwoStepsBack<1> = 1;
    expect(_).toEqual(_);
  });
  it("ThreeStepsForwardTwoStepsBack<2>", () => {
    const _: ThreeStepsForwardTwoStepsBack<2> = 2;
    expect(_).toEqual(_);
  });
  it("ThreeStepsForwardTwoStepsBack<3>", () => {
    const _: ThreeStepsForwardTwoStepsBack<3> = 3;
    expect(_).toEqual(_);
  });
  it("ThreeStepsForwardTwoStepsBack<4>", () => {
    const _: ThreeStepsForwardTwoStepsBack<4> = 6;
    expect(_).toEqual(_);
  });
});
