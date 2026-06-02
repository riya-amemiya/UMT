import { bench, run, summary, do_not_optimize } from "mitata";
import { sumPrecise } from "@/Math/sumPrecise";

const smallArray = Array.from({ length: 100 }, () => Math.random());
const mediumArray = Array.from({ length: 10_000 }, () => Math.random());
const largeArray = Array.from({ length: 1_000_000 }, () => Math.random());

const reduceSum = (numbers: number[]): number =>
  numbers.reduce((acc, n) => acc + n, 0);

// Summing the smaller arrays runs in well under a microsecond, which is buried
// in CI-runner jitter. Each size repeats its work a fixed number of times so
// that one measured sample reaches a few milliseconds and stays above that
// jitter. The count is shared between sumPrecise and reduce within a size so
// their comparison stays fair.
const SMALL_ITERATIONS = 40_000;
const MEDIUM_ITERATIONS = 400;
const LARGE_ITERATIONS = 5;

summary(() => {
  bench("sumPrecise (100 elements)", () => {
    for (let i = 0; i < SMALL_ITERATIONS; i++) {
      do_not_optimize(sumPrecise(smallArray));
    }
  });

  bench("Array.reduce (100 elements)", () => {
    for (let i = 0; i < SMALL_ITERATIONS; i++) {
      do_not_optimize(reduceSum(smallArray));
    }
  });

  bench("sumPrecise (10,000 elements)", () => {
    for (let i = 0; i < MEDIUM_ITERATIONS; i++) {
      do_not_optimize(sumPrecise(mediumArray));
    }
  });

  bench("Array.reduce (10,000 elements)", () => {
    for (let i = 0; i < MEDIUM_ITERATIONS; i++) {
      do_not_optimize(reduceSum(mediumArray));
    }
  });

  bench("sumPrecise (1,000,000 elements)", () => {
    for (let i = 0; i < LARGE_ITERATIONS; i++) {
      do_not_optimize(sumPrecise(largeArray));
    }
  });

  bench("Array.reduce (1,000,000 elements)", () => {
    for (let i = 0; i < LARGE_ITERATIONS; i++) {
      do_not_optimize(reduceSum(largeArray));
    }
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
