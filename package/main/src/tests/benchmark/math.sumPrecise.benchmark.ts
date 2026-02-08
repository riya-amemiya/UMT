import { bench, run, summary, do_not_optimize } from "mitata";
import { sumPrecise } from "@/Math/sumPrecise";

const smallArray = Array.from({ length: 100 }, () => Math.random());
const mediumArray = Array.from({ length: 10_000 }, () => Math.random());
const largeArray = Array.from({ length: 1_000_000 }, () => Math.random());

const reduceSum = (numbers: number[]): number =>
  numbers.reduce((acc, n) => acc + n, 0);

summary(() => {
  bench("sumPrecise (100 elements)", () => {
    do_not_optimize(sumPrecise(smallArray));
  });

  bench("Array.reduce (100 elements)", () => {
    do_not_optimize(reduceSum(smallArray));
  });

  bench("sumPrecise (10,000 elements)", () => {
    do_not_optimize(sumPrecise(mediumArray));
  });

  bench("Array.reduce (10,000 elements)", () => {
    do_not_optimize(reduceSum(mediumArray));
  });

  bench("sumPrecise (1,000,000 elements)", () => {
    do_not_optimize(sumPrecise(largeArray));
  });

  bench("Array.reduce (1,000,000 elements)", () => {
    do_not_optimize(reduceSum(largeArray));
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
