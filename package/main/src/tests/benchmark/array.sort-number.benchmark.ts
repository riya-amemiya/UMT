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
import { ultraSort } from "@/Array/ultraSort";
import { ultraNumberSort } from "@/Array/ultraNumberSort";

const compareFunction = (a: number, b: number): number => a - b;

const arraySizes = [10, 100, 1000, 10000, 100000, 1000000, 10000000];

const sharedRandomArrays = new Map<number, number[]>();
for (const size of arraySizes) {
  sharedRandomArrays.set(
    size,
    Array.from({ length: size }, () => Math.random() * size),
  );
}

summary(() => {
  lineplot(() => {
    bench("quickSort($size)", function* quickSortBench(state: k_state) {
      const size = state.get("size") as number;
      const original_array = sharedRandomArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        [0]() {
          return [...original_array];
        },
        bench(arr: number[]) {
          do_not_optimize(quickSort(arr, compareFunction));
        },
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench(
      "Array.prototype.sort($size)",
      function* nativeSortBench(state: k_state) {
        const size = state.get("size") as number;
        const original_array = sharedRandomArrays.get(size);

        if (!original_array) {
          throw new Error(`No shared array found for size: ${size}`);
        }

        yield {
          [0]() {
            return [...original_array];
          },
          bench(arr: number[]) {
            do_not_optimize(arr.sort(compareFunction));
          },
        };
      },
    )
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

        yield {
          [0]() {
            return [...original_array];
          },
          bench(arr: number[]) {
            do_not_optimize(dualPivotQuickSort(arr, compareFunction));
          },
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

      yield {
        [0]() {
          return [...original_array];
        },
        bench(arr: number[]) {
          do_not_optimize(sort(arr).asc());
        },
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("ultraSort($size)", function* ultraSortBench(state: k_state) {
      const size = state.get("size") as number;
      const original_array = sharedRandomArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        [0]() {
          return [...original_array];
        },
        bench(arr: number[]) {
          do_not_optimize(ultraSort(arr, compareFunction));
        },
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

        yield {
          [0]() {
            return [...original_array];
          },
          bench(arr: number[]) {
            do_not_optimize(ultraNumberSort(arr, true));
          },
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
