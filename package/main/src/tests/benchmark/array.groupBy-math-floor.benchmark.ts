import {
  run,
  bench,
  summary,
  lineplot,
  do_not_optimize,
  type k_state,
} from "mitata";
import { groupBy as customGroupBy } from "@/Array/groupBy";
import { groupBy as lodashGroupBy } from "lodash";
import { groupBy as esToolkitGroupBy } from "es-toolkit";

const arraySizes = [1000, 10_000, 100_000, 1_000_000, 10_000_000];

const sharedRandomNumbersArrays = new Map<number, number[]>();

for (const size of arraySizes) {
  sharedRandomNumbersArrays.set(
    size,
    Array.from({ length: size }, (_, i) => Math.random() * 100 + i),
  );
}

const byMathFloor = (item: number): number => Math.floor(item);

// Small/medium inputs run in microseconds, which is buried in CI-runner jitter
// and produces meaningless base/head diffs. Repeating the operation a fixed
// number of times lifts one measured sample to a few milliseconds so it stays
// well above that jitter. The count is fixed (not derived from measured speed)
// so the comparison still reflects real per-operation differences. From 100k
// upward a single call already takes long enough and runs once.
const repsForSize = (size: number): number => {
  if (size <= 1000) {
    return 100;
  }
  if (size <= 10_000) {
    return 10;
  }
  return 1;
};

summary(() => {
  lineplot(() => {
    bench(
      "customGroupBy numbers by Math.floor (size: $size)",
      function* (state: k_state) {
        const size = state.get("size") as number;
        const originalArray = sharedRandomNumbersArrays.get(size);
        if (!originalArray) {
          throw new Error(`No shared array found for size: ${size}`);
        }
        const reps = repsForSize(size);
        yield () => {
          for (let r = 0; r < reps; r++) {
            do_not_optimize(customGroupBy(originalArray, byMathFloor));
          }
        };
      },
    )
      .args("size", arraySizes)
      .gc("inner");

    bench(
      "lodashGroupBy numbers by Math.floor (size: $size)",
      function* (state: k_state) {
        const size = state.get("size") as number;
        const originalArray = sharedRandomNumbersArrays.get(size);
        if (!originalArray) {
          throw new Error(`No shared array found for size: ${size}`);
        }
        const reps = repsForSize(size);
        yield () => {
          for (let r = 0; r < reps; r++) {
            do_not_optimize(lodashGroupBy(originalArray, byMathFloor));
          }
        };
      },
    )
      .args("size", arraySizes)
      .gc("inner");

    bench(
      "esToolkitGroupBy numbers by Math.floor (size: $size)",
      function* (state: k_state) {
        const size = state.get("size") as number;
        const originalArray = sharedRandomNumbersArrays.get(size);
        if (!originalArray) {
          throw new Error(`No shared array found for size: ${size}`);
        }
        const reps = repsForSize(size);
        yield () => {
          for (let r = 0; r < reps; r++) {
            do_not_optimize(esToolkitGroupBy(originalArray, byMathFloor));
          }
        };
      },
    )
      .args("size", arraySizes)
      .gc("inner");

    bench(
      "Object.groupBy numbers by Math.floor (size: $size)",
      function* (state: k_state) {
        const size = state.get("size") as number;
        const originalArray = sharedRandomNumbersArrays.get(size);
        if (!originalArray) {
          throw new Error(`No shared array found for size: ${size}`);
        }
        const reps = repsForSize(size);
        yield () => {
          for (let r = 0; r < reps; r++) {
            do_not_optimize(Object.groupBy(originalArray, byMathFloor));
          }
        };
      },
    )
      .args("size", arraySizes)
      .gc("inner");
  });
});

(async () => {
  try {
    await run();
    console.log("Benchmark for byMathFloor finished successfully.");
  } catch (e) {
    console.error("Error during benchmark execution (byMathFloor):", e);
  }
})();
