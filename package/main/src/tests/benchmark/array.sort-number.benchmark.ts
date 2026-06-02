import {
  run,
  bench,
  summary,
  lineplot,
  do_not_optimize,
  type k_state,
} from "mitata";
import { quickSort } from "@/Array/quickSort";
import { dualPivotQuickSort } from "@/Array/dualPivotQuickSort";
import { sort } from "fast-sort";
import { ultraNumberSort } from "@/Array/ultraNumberSort";
import { timSort } from "@/Array/timSort";
import { mergeSort } from "@/Array/mergeSort";

const compareFunction = (a: number, b: number): number => a - b;

const arraySizes = [10, 100, 1000, 10_000, 100_000, 1_000_000, 10_000_000];

const sharedRandomArrays = new Map<number, number[]>();
for (const size of arraySizes) {
  sharedRandomArrays.set(
    size,
    Array.from({ length: size }, () => Math.random() * size),
  );
}

// Small arrays sort in nanoseconds to a few microseconds, which is buried in
// CI-runner jitter and produces meaningless base/head diffs. Repeating the sort
// a fixed number of times lifts one measured sample to a few milliseconds so it
// stays well above that jitter. The count is fixed (not derived from measured
// speed) so the comparison still reflects real per-operation differences. Only
// the small sizes are repeated: from 10k upward a single sort already takes
// long enough, and some algorithms become pathologically slow on adversarial
// inputs, so repeating them there would explode the run time.
const repsForSize = (size: number): number => {
  if (size <= 10) {
    return 20_000;
  }
  if (size <= 100) {
    return 2000;
  }
  if (size <= 1000) {
    return 200;
  }
  return 1;
};

summary(() => {
  lineplot(() => {
    bench(
      "Array.prototype.sort($size)",
      function* nativeSortBench(state: k_state) {
        const size = state.get("size") as number;
        const original_array = sharedRandomArrays.get(size);

        if (!original_array) {
          throw new Error(`No shared array found for size: ${size}`);
        }

        const reps = repsForSize(size);
        yield () => {
          for (let r = 0; r < reps; r++) {
            do_not_optimize([...original_array].sort(compareFunction));
          }
        };
      },
    )
      .args("size", arraySizes)
      .gc("inner");

    bench("fast-sort($size)", function* fastSortBench(state: k_state) {
      const size = state.get("size") as number;
      const original_array = sharedRandomArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      const reps = repsForSize(size);
      yield () => {
        for (let r = 0; r < reps; r++) {
          do_not_optimize(sort([...original_array]).asc());
        }
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("quickSort($size)", function* quickSortBench(state: k_state) {
      const size = state.get("size") as number;
      const original_array = sharedRandomArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      const reps = repsForSize(size);
      yield () => {
        for (let r = 0; r < reps; r++) {
          do_not_optimize(quickSort([...original_array], compareFunction));
        }
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench(
      "dualPivotQuickSort($size)",
      function* dualPivotQuickSortBench(state: k_state) {
        const size = state.get("size") as number;
        const original_array = sharedRandomArrays.get(size);

        if (!original_array) {
          throw new Error(`No shared array found for size: ${size}`);
        }

        const reps = repsForSize(size);
        yield () => {
          for (let r = 0; r < reps; r++) {
            do_not_optimize(
              dualPivotQuickSort([...original_array], compareFunction),
            );
          }
        };
      },
    )
      .args("size", arraySizes)
      .gc("inner");

    bench("timSort($size)", function* timSortBench(state: k_state) {
      const size = state.get("size") as number;
      const original_array = sharedRandomArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      const reps = repsForSize(size);
      yield () => {
        for (let r = 0; r < reps; r++) {
          do_not_optimize(timSort([...original_array], compareFunction));
        }
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("mergeSort($size)", function* mergeSortBench(state: k_state) {
      const size = state.get("size") as number;
      const original_array = sharedRandomArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      const reps = repsForSize(size);
      yield () => {
        for (let r = 0; r < reps; r++) {
          do_not_optimize(mergeSort([...original_array], compareFunction));
        }
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench(
      "ultraNumberSort($size)",
      function* ultraNumberSortBench(state: k_state) {
        const size = state.get("size") as number;
        const original_array = sharedRandomArrays.get(size);

        if (!original_array) {
          throw new Error(`No shared array found for size: ${size}`);
        }

        const reps = repsForSize(size);
        yield () => {
          for (let r = 0; r < reps; r++) {
            do_not_optimize(ultraNumberSort([...original_array], true));
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
    console.log("Benchmark finished.");
  } catch (e) {
    console.error("Error during benchmark execution:", e);
  }
})();
