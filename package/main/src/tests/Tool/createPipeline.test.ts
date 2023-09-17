import { createPipeline } from "@/Tool/createPipeline";

test("createPipeline", () => {
  expect(createPipeline(3)((x) => x + 1)((x) => x * 2)()).toBe(8);
  expect(createPipeline(3)((x) => x + 1)((x) => x * 2)((x) => x - 1)()).toBe(7);
});
