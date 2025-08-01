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

summary(() => {
  lineplot(() => {
    bench("ultraNumberSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = almostSortedArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        0() {
          return [...original_array];
        },
        bench(arr: number[]) {
          do_not_optimize(ultraNumberSort(arr));
        },
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

      yield {
        0() {
          return [...original_array];
        },
        bench(arr: number[]) {
          do_not_optimize(quickSort(arr, compareFunction));
        },
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

      yield {
        0() {
          return [...original_array];
        },
        bench(arr: number[]) {
          do_not_optimize(dualPivotQuickSort(arr, compareFunction));
        },
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

      yield {
        0() {
          return [...original_array];
        },
        bench(arr: number[]) {
          do_not_optimize(mergeSort(arr, compareFunction));
        },
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

      yield {
        0() {
          return [...original_array];
        },
        bench(arr: number[]) {
          do_not_optimize(timSort(arr, compareFunction));
        },
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

      yield {
        0() {
          return [...original_array];
        },
        bench(arr: number[]) {
          do_not_optimize(arr.sort(compareFunction));
        },
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
