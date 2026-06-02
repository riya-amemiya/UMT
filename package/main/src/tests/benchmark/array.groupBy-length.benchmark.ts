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

const sharedRandomStringsArraysByLength = new Map<number, string[]>();

for (const size of arraySizes) {
  sharedRandomStringsArraysByLength.set(
    size,
    Array.from(
      { length: size },
      (_, i) => `item${i}-${Math.random().toString(36).slice(2)}`,
    ),
  );
}

const byLength = (item: string): number => item.length;

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
      "customGroupBy strings by length (size: $size)",
      function* (state: k_state) {
        const size = state.get("size") as number;
        const originalArray = sharedRandomStringsArraysByLength.get(size);
        if (!originalArray) {
          throw new Error(`No shared array found for size: ${size}`);
        }
        const reps = repsForSize(size);
        yield () => {
          for (let r = 0; r < reps; r++) {
            do_not_optimize(customGroupBy(originalArray, byLength));
          }
        };
      },
    )
      .args("size", arraySizes)
      .gc("inner");

    bench(
      "lodashGroupBy strings by length (size: $size)",
      function* (state: k_state) {
        const size = state.get("size") as number;
        const originalArray = sharedRandomStringsArraysByLength.get(size);
        if (!originalArray) {
          throw new Error(`No shared array found for size: ${size}`);
        }
        const reps = repsForSize(size);
        yield () => {
          for (let r = 0; r < reps; r++) {
            do_not_optimize(lodashGroupBy(originalArray, byLength));
          }
        };
      },
    )
      .args("size", arraySizes)
      .gc("inner");

    bench(
      "esToolkitGroupBy strings by length (size: $size)",
      function* (state: k_state) {
        const size = state.get("size") as number;
        const originalArray = sharedRandomStringsArraysByLength.get(size);
        if (!originalArray) {
          throw new Error(`No shared array found for size: ${size}`);
        }
        const reps = repsForSize(size);
        yield () => {
          for (let r = 0; r < reps; r++) {
            do_not_optimize(esToolkitGroupBy(originalArray, byLength));
          }
        };
      },
    )
      .args("size", arraySizes)
      .gc("inner");

    bench(
      "Object.groupBy strings by length (size: $size)",
      function* (state: k_state) {
        const size = state.get("size") as number;
        const originalArray = sharedRandomStringsArraysByLength.get(size);
        if (!originalArray) {
          throw new Error(`No shared array found for size: ${size}`);
        }
        const reps = repsForSize(size);
        yield () => {
          for (let r = 0; r < reps; r++) {
            do_not_optimize(Object.groupBy(originalArray, byLength));
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
    console.log("Benchmark for byLength finished successfully.");
  } catch (e) {
    console.error("Error during benchmark execution (byLength):", e);
  }
})();
