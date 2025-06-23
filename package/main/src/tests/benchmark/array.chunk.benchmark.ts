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

const arraySizes = [1000, 10000, 100000, 1000000, 10000000];
const chunkSizes = [2, 5, 10, 20, 50, 100];

const sharedRandomArrays = new Map<number, number[]>();
for (const size of arraySizes) {
  sharedRandomArrays.set(
    size,
    Array.from({ length: size }, () => Math.random() * size),
  );
}

summary(() => {
  lineplot(() => {
    bench(
      `customChunk(size: $size, chunkSize: $chunkSize)`,
      function* customChunkBench(state: k_state) {
        const size = state.get("size") as number;
        const currentChunkSize = state.get("chunkSize") as number;
        const original_array = sharedRandomArrays.get(size);

        if (!original_array) {
          throw new Error(`No shared array found for size: ${size}`);
        }

        yield {
          [0]() {
            return [...original_array];
          },
          bench(arr: number[]) {
            do_not_optimize(customChunk(arr, currentChunkSize));
          },
        };
      },
    )
      .args({ size: arraySizes, chunkSize: chunkSizes })
      .gc("inner");

    bench(
      `lodashChunk(size: $size, chunkSize: $chunkSize)`,
      function* lodashChunkBench(state: k_state) {
        const size = state.get("size") as number;
        const currentChunkSize = state.get("chunkSize") as number;
        const original_array = sharedRandomArrays.get(size);

        if (!original_array) {
          throw new Error(`No shared array found for size: ${size}`);
        }

        yield {
          [0]() {
            return [...original_array];
          },
          bench(arr: number[]) {
            do_not_optimize(lodashChunk(arr, currentChunkSize));
          },
        };
      },
    )
      .args({ size: arraySizes, chunkSize: chunkSizes })
      .gc("inner");

    bench(
      `esToolkitChunk(size: $size, chunkSize: $chunkSize)`,
      function* esToolkitChunkBench(state: k_state) {
        const size = state.get("size") as number;
        const currentChunkSize = state.get("chunkSize") as number;
        const original_array = sharedRandomArrays.get(size);

        if (!original_array) {
          throw new Error(`No shared array found for size: ${size}`);
        }

        yield {
          [0]() {
            return [...original_array];
          },
          bench(arr: number[]) {
            do_not_optimize(esToolkitChunk(arr, currentChunkSize));
          },
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
