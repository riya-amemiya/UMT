import {
  run,
  bench,
  summary,
  lineplot,
  do_not_optimize,
  type k_state,
} from "mitata";
import { chunk as customChunk } from "@/Array/chunk";
import { chunk as lodashChunk } from "lodash";
import { chunk as esToolkitChunk } from "es-toolkit";

const arraySizes = [1000, 10_000, 100_000, 1_000_000, 10_000_000];
const chunkSizes = [2, 5, 10, 20, 50, 100];

const sharedRandomArrays = new Map<number, number[]>();
for (const size of arraySizes) {
  sharedRandomArrays.set(
    size,
    Array.from({ length: size }, () => Math.random() * size),
  );
}

// Small/medium inputs run in microseconds, which is buried in CI-runner jitter
// and produces meaningless base/head diffs. Repeating the operation a fixed
// number of times lifts one measured sample to a few milliseconds so it stays
// well above that jitter. The count is fixed (not derived from measured speed)
// so the comparison still reflects real per-operation differences. From 100k
// upward a single call already takes long enough and runs once.
const repsForSize = (size: number): number => {
  if (size <= 1000) {
    return 300;
  }
  if (size <= 10_000) {
    return 30;
  }
  return 1;
};

summary(() => {
  lineplot(() => {
    bench(
      "customChunk(size: $size, chunkSize: $chunkSize)",
      function* customChunkBench(state: k_state) {
        const size = state.get("size") as number;
        const currentChunkSize = state.get("chunkSize") as number;
        const original_array = sharedRandomArrays.get(size);

        if (!original_array) {
          throw new Error(`No shared array found for size: ${size}`);
        }

        const reps = repsForSize(size);
        yield () => {
          for (let r = 0; r < reps; r++) {
            do_not_optimize(customChunk(original_array, currentChunkSize));
          }
        };
      },
    )
      .args({ size: arraySizes, chunkSize: chunkSizes })
      .gc("inner");

    bench(
      "lodashChunk(size: $size, chunkSize: $chunkSize)",
      function* lodashChunkBench(state: k_state) {
        const size = state.get("size") as number;
        const currentChunkSize = state.get("chunkSize") as number;
        const original_array = sharedRandomArrays.get(size);

        if (!original_array) {
          throw new Error(`No shared array found for size: ${size}`);
        }

        const reps = repsForSize(size);
        yield () => {
          for (let r = 0; r < reps; r++) {
            do_not_optimize(lodashChunk(original_array, currentChunkSize));
          }
        };
      },
    )
      .args({ size: arraySizes, chunkSize: chunkSizes })
      .gc("inner");

    bench(
      "esToolkitChunk(size: $size, chunkSize: $chunkSize)",
      function* esToolkitChunkBench(state: k_state) {
        const size = state.get("size") as number;
        const currentChunkSize = state.get("chunkSize") as number;
        const original_array = sharedRandomArrays.get(size);

        if (!original_array) {
          throw new Error(`No shared array found for size: ${size}`);
        }

        const reps = repsForSize(size);
        yield () => {
          for (let r = 0; r < reps; r++) {
            do_not_optimize(esToolkitChunk(original_array, currentChunkSize));
          }
        };
      },
    )
      .args({ size: arraySizes, chunkSize: chunkSizes })
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
