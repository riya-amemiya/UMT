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

const sharedRandomStringsArraysByFirstChar = new Map<number, string[]>();

for (const size of arraySizes) {
  const chars = "abcdefghijklmnopqrstuvwxyz";
  sharedRandomStringsArraysByFirstChar.set(
    size,
    Array.from(
      { length: size },
      (_, i) =>
        chars.charAt(Math.floor(Math.random() * chars.length)) +
        `item${i}-${Math.random().toString(36).substring(2)}`,
    ),
  );
}

const byFirstChar = (item: string): string => item[0];

summary(() => {
  lineplot(() => {
    bench(
      `customGroupBy strings by first char (size: $size)`,
      function* (state: k_state) {
        const size = state.get("size") as number;
        const originalArray = sharedRandomStringsArraysByFirstChar.get(size);
        if (!originalArray) {
          throw new Error(`No shared array found for size: ${size}`);
        }
        yield {
          [0]() {
            return [...originalArray];
          },
          bench(arr: string[]) {
            do_not_optimize(customGroupBy(arr, byFirstChar));
          },
        };
      },
    )
      .args("size", arraySizes)
      .gc("inner");

    bench(
      `lodashGroupBy strings by first char (size: $size)`,
      function* (state: k_state) {
        const size = state.get("size") as number;
        const originalArray = sharedRandomStringsArraysByFirstChar.get(size);
        if (!originalArray) {
          throw new Error(`No shared array found for size: ${size}`);
        }
        yield {
          [0]() {
            return [...originalArray];
          },
          bench(arr: string[]) {
            do_not_optimize(lodashGroupBy(arr, byFirstChar));
          },
        };
      },
    )
      .args("size", arraySizes)
      .gc("inner");

    bench(
      `esToolkitGroupBy strings by first char (size: $size)`,
      function* (state: k_state) {
        const size = state.get("size") as number;
        const originalArray = sharedRandomStringsArraysByFirstChar.get(size);
        if (!originalArray) {
          throw new Error(`No shared array found for size: ${size}`);
        }
        yield {
          [0]() {
            return [...originalArray];
          },
          bench(arr: string[]) {
            do_not_optimize(esToolkitGroupBy(arr, byFirstChar));
          },
        };
      },
    )
      .args("size", arraySizes)
      .gc("inner");

    bench(
      `Object.groupBy strings by first char (size: $size)`,
      function* (state: k_state) {
        const size = state.get("size") as number;
        const originalArray = sharedRandomStringsArraysByFirstChar.get(size);
        if (!originalArray) {
          throw new Error(`No shared array found for size: ${size}`);
        }
        yield {
          [0]() {
            return [...originalArray];
          },
          bench(arr: string[]) {
            do_not_optimize(Object.groupBy(arr, byFirstChar));
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
    console.log("Benchmark for byFirstChar finished successfully.");
  } catch (e) {
    console.error("Error during benchmark execution (byFirstChar):", e);
  }
})();
