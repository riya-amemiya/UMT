import {
  run,
  bench,
  summary,
  lineplot,
  do_not_optimize,
  type k_state,
} from "mitata";
import { getArraysCommon } from "@/Array/getArraysCommon";

const arraySizes = [100, 1000, 10_000];
const numberOfArrays = [2, 5, 10];

const sharedRandomArrays = new Map<number, number[][]>();

for (const size of arraySizes) {
  // Generate a base array
  const baseArray = Array.from({ length: size }, () =>
    Math.floor(Math.random() * size),
  );

  // Create variations for "other arrays"
  // We want some intersection, so we'll mix the base array with some new random numbers
  const arrays: number[][] = [];
  // Max needed is 10 arrays (from numberOfArrays)
  for (let i = 0; i < 10; i++) {
    arrays.push(
      Array.from({ length: size }, () =>
        Math.random() > 0.5
          ? Math.floor(Math.random() * size)
          : baseArray[Math.floor(Math.random() * size)],
      ),
    );
  }

  // Store them. Key is size. We store [baseArray, ...otherArrays]
  sharedRandomArrays.set(size, [baseArray, ...arrays]);
}

summary(() => {
  lineplot(() => {
    bench(
      "getArraysCommon(size: $size, arrays: $count)",
      function* commonBench(state: k_state) {
        const size = state.get("size") as number;
        const count = state.get("count") as number;

        const allArrays = sharedRandomArrays.get(size);
        if (!allArrays) {
          throw new Error(`No shared array found for size: ${size}`);
        }

        // The first array is the primary one, then we take `count - 1` additional arrays
        const inputArrays = allArrays.slice(0, count);
        const [first, ...rest] = inputArrays;

        yield {
          0() {
            return inputArrays;
          },
          bench() {
            do_not_optimize(getArraysCommon(first, ...rest));
          },
        };
      },
    )
      .args({ size: arraySizes, count: numberOfArrays })
      .gc("inner");
  });
});

(async () => {
  try {
    await run();
    console.log("Benchmark finished successfully.");
  } catch (e) {
    console.error("Error during benchmark execution:", e);
  }
})();
