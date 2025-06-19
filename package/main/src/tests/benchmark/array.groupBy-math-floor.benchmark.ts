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

const arraySizes = [1000, 10000, 100000, 1000000, 10000000];

const sharedRandomNumbersArrays = new Map<number, number[]>();

for (const size of arraySizes) {
  sharedRandomNumbersArrays.set(
    size,
    Array.from({ length: size }, (_, i) => Math.random() * 100 + i),
  );
}

const byMathFloor = (item: number): number => Math.floor(item);

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
        yield {
          0() {
            return [...originalArray];
          },
          bench(arr: number[]) {
            do_not_optimize(customGroupBy(arr, byMathFloor));
          },
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
        yield {
          0() {
            return [...originalArray];
          },
          bench(arr: number[]) {
            do_not_optimize(lodashGroupBy(arr, byMathFloor));
          },
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
        yield {
          0() {
            return [...originalArray];
          },
          bench(arr: number[]) {
            do_not_optimize(esToolkitGroupBy(arr, byMathFloor));
          },
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
        yield {
          0() {
            return [...originalArray];
          },
          bench(arr: number[]) {
            do_not_optimize(Object.groupBy(arr, byMathFloor));
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
    console.log("Benchmark for byMathFloor finished successfully.");
  } catch (e) {
    console.error("Error during benchmark execution (byMathFloor):", e);
  }
})();
