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
import { ultraNumberSort } from "@/Array/ultraNumberSort";
import { mergeSort } from "@/Array/mergeSort";
import { timSort } from "@/Array/timSort";

const compareFunction = (a: number, b: number): number => a - b;

const arraySizes = [10, 100, 1000, 10_000, 100_000];

// Almost sorted (few elements out of place)
const almostSortedArrays = new Map<number, number[]>();

for (const size of arraySizes) {
  const almostSorted = Array.from({ length: size }, (_, i) => i);
  // Swap some random elements
  for (let i = 0; i < size / 10; i++) {
    const idx1 = Math.floor(Math.random() * size);
    const idx2 = Math.floor(Math.random() * size);
    [almostSorted[idx1], almostSorted[idx2]] = [
      almostSorted[idx2],
      almostSorted[idx1],
    ];
  }
  almostSortedArrays.set(size, almostSorted);
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
    bench("ultraNumberSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = almostSortedArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      const reps = repsForSize(size);
      yield () => {
        for (let r = 0; r < reps; r++) {
          const arr: number[] = [...original_array];
          do_not_optimize(ultraNumberSort(arr));
        }
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("quickSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = almostSortedArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      const reps = repsForSize(size);
      yield () => {
        for (let r = 0; r < reps; r++) {
          const arr: number[] = [...original_array];
          do_not_optimize(quickSort(arr, compareFunction));
        }
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("dualPivotQuickSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = almostSortedArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      const reps = repsForSize(size);
      yield () => {
        for (let r = 0; r < reps; r++) {
          const arr: number[] = [...original_array];
          do_not_optimize(dualPivotQuickSort(arr, compareFunction));
        }
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("mergeSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = almostSortedArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      const reps = repsForSize(size);
      yield () => {
        for (let r = 0; r < reps; r++) {
          const arr: number[] = [...original_array];
          do_not_optimize(mergeSort(arr, compareFunction));
        }
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("timSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = almostSortedArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      const reps = repsForSize(size);
      yield () => {
        for (let r = 0; r < reps; r++) {
          const arr: number[] = [...original_array];
          do_not_optimize(timSort(arr, compareFunction));
        }
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("Array.sort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = almostSortedArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      const reps = repsForSize(size);
      yield () => {
        for (let r = 0; r < reps; r++) {
          const arr: number[] = [...original_array];
          do_not_optimize(arr.sort(compareFunction));
        }
      };
    })
      .args("size", arraySizes)
      .gc("inner");
  });
});

(async () => {
  try {
    await run();
    console.log("Almost sorted array benchmark finished.");
  } catch (e) {
    console.error("Error during benchmark execution:", e);
  }
})();
