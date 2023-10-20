import { createPipeline } from "@/Tool/createPipeline";

describe("createPipeline", () => {
  it("should return a pipeline", () => {
    const pipeline = createPipeline(1);
    expect(pipeline()).toBe(1);
  });

  it("should return a new pipeline", () => {
    const pipeline = createPipeline(1);
    const newPipeline = pipeline((x) => x + 1);
    expect(newPipeline()).toBe(2);
  });
});
