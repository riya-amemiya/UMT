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
import { timSort } from "@/Array/timSort";
import { mergeSort } from "@/Array/mergeSort";
import { insertionSort } from "@/Array/insertionSort";

const compareFunction = (a: string, b: string): number => a.localeCompare(b);

const arraySizes = [10, 100, 1000, 10000, 100000];

// Generate random strings
const generateRandomString = (length: number): string => {
  const chars =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  let result = "";
  for (let i = 0; i < length; i++) {
    result += chars.charAt(Math.floor(Math.random() * chars.length));
  }
  return result;
};

// Short strings (5-10 chars)
const shortStringArrays = new Map<number, string[]>();

for (const size of arraySizes) {
  shortStringArrays.set(
    size,
    Array.from({ length: size }, () =>
      generateRandomString(5 + Math.floor(Math.random() * 6)),
    ),
  );
}

summary(() => {
  lineplot(() => {
    bench("quickSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = shortStringArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        [0]() {
          return [...original_array];
        },
        bench(arr: string[]) {
          do_not_optimize(quickSort(arr, compareFunction));
        },
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("dualPivotQuickSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = shortStringArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        [0]() {
          return [...original_array];
        },
        bench(arr: string[]) {
          do_not_optimize(dualPivotQuickSort(arr, compareFunction));
        },
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("timSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = shortStringArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        [0]() {
          return [...original_array];
        },
        bench(arr: string[]) {
          do_not_optimize(timSort(arr, compareFunction));
        },
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("mergeSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = shortStringArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        [0]() {
          return [...original_array];
        },
        bench(arr: string[]) {
          do_not_optimize(mergeSort(arr, compareFunction));
        },
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("insertionSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = shortStringArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        [0]() {
          return [...original_array];
        },
        bench(arr: string[]) {
          do_not_optimize(insertionSort(arr, compareFunction));
        },
      };
    })
      .args("size", arraySizes.slice(0, 3)) // Only small sizes for insertion sort
      .gc("inner");

    bench("Array.sort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = shortStringArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        [0]() {
          return [...original_array];
        },
        bench(arr: string[]) {
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
    console.log("Short string sort benchmark finished.");
  } catch (e) {
    console.error("Error during benchmark execution:", e);
  }
})();
