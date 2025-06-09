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

const sharedRandomStringsArraysByLength = new Map<number, string[]>();

for (const size of arraySizes) {
  sharedRandomStringsArraysByLength.set(
    size,
    Array.from(
      { length: size },
      (_, i) => `item${i}-${Math.random().toString(36).substring(2)}`,
    ),
  );
}

const byLength = (item: string): number => item.length;

summary(() => {
  lineplot(() => {
    bench(
      `customGroupBy strings by length (size: $size)`,
      function* (state: k_state) {
        const size = state.get("size") as number;
        const originalArray = sharedRandomStringsArraysByLength.get(size);
        if (!originalArray) {
          throw new Error(`No shared array found for size: ${size}`);
        }
        yield {
          [0]() {
            return [...originalArray];
          },
          bench(arr: string[]) {
            do_not_optimize(customGroupBy(arr, byLength));
          },
        };
      },
    )
      .args("size", arraySizes)
      .gc("inner");

    bench(
      `lodashGroupBy strings by length (size: $size)`,
      function* (state: k_state) {
        const size = state.get("size") as number;
        const originalArray = sharedRandomStringsArraysByLength.get(size);
        if (!originalArray) {
          throw new Error(`No shared array found for size: ${size}`);
        }
        yield {
          [0]() {
            return [...originalArray];
          },
          bench(arr: string[]) {
            do_not_optimize(lodashGroupBy(arr, byLength));
          },
        };
      },
    )
      .args("size", arraySizes)
      .gc("inner");

    bench(
      `esToolkitGroupBy strings by length (size: $size)`,
      function* (state: k_state) {
        const size = state.get("size") as number;
        const originalArray = sharedRandomStringsArraysByLength.get(size);
        if (!originalArray) {
          throw new Error(`No shared array found for size: ${size}`);
        }
        yield {
          [0]() {
            return [...originalArray];
          },
          bench(arr: string[]) {
            do_not_optimize(esToolkitGroupBy(arr, byLength));
          },
        };
      },
    )
      .args("size", arraySizes)
      .gc("inner");

    bench(
      `Object.groupBy strings by length (size: $size)`,
      function* (state: k_state) {
        const size = state.get("size") as number;
        const originalArray = sharedRandomStringsArraysByLength.get(size);
        if (!originalArray) {
          throw new Error(`No shared array found for size: ${size}`);
        }
        yield {
          [0]() {
            return [...originalArray];
          },
          bench(arr: string[]) {
            do_not_optimize(Object.groupBy(arr, byLength));
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
    console.log("Benchmark for byLength finished successfully.");
  } catch (e) {
    console.error("Error during benchmark execution (byLength):", e);
  }
})();
