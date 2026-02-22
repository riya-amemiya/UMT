import { bench, run, summary, do_not_optimize } from "mitata";
import { generateNumberArray } from "@/Array/generateNumberArray";

summary(() => {
  // Linear generation
  bench("generateNumberArray (100 elements)", () => {
    do_not_optimize(generateNumberArray(100));
  });

  bench("generateNumberArray (1000 elements)", () => {
    do_not_optimize(generateNumberArray(1000));
  });

  bench("generateNumberArray (10,000 elements)", () => {
    do_not_optimize(generateNumberArray(10_000));
  });

  // Random generation
  bench("generateNumberArray random (100 elements)", () => {
    do_not_optimize(generateNumberArray(100, 0, 100, true));
  });

  bench("generateNumberArray random (1000 elements)", () => {
    do_not_optimize(generateNumberArray(1000, 0, 1000, true));
  });

  bench("generateNumberArray random (10,000 elements)", () => {
    do_not_optimize(generateNumberArray(10_000, 0, 10_000, true));
  });
});

(async () => {
  try {
    await run();
    console.log("Benchmark finished successfully.");
  } catch (e) {
    console.error("Error during benchmark execution:", e);
  }
})();
