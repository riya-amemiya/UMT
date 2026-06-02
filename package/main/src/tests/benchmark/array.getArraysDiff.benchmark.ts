import {
  run,
  bench,
  summary,
  lineplot,
  do_not_optimize,
  type k_state,
} from "mitata";
import { getArraysDiff } from "../../Array/getArraysDiff";

const arraySizes = [100, 1000, 10_000];
const numberOfArrays = [2, 5];

// Small inputs run in microseconds, which is buried in CI-runner jitter and
// produces meaningless base/head diffs. Repeating the operation a fixed number
// of times lifts one measured sample to a few milliseconds so it stays well
// above that jitter. The count is fixed (not derived from measured speed) so
// the comparison still reflects real per-operation differences.
const repsForSize = (size: number): number => {
  if (size <= 100) {
    return 1500;
  }
  if (size <= 1000) {
    return 150;
  }
  return 15;
};

summary(() => {
  lineplot(() => {
    bench(
      "getArraysDiff(size: $size, arrays: $count)",
      function* (state: k_state) {
        const size = state.get("size");
        const count = state.get("count");

        const arrays: number[][] = [];
        const base = Array.from({ length: size }, (_, i) => i);
        arrays.push(base);
        for (let i = 1; i < count; i++) {
          const arr = Array.from({ length: size }, (_, j) =>
            j % 2 === 0 ? j : j + size,
          );
          arrays.push(arr);
        }

        const reps = repsForSize(size);
        yield () => {
          for (let r = 0; r < reps; r++) {
            do_not_optimize(getArraysDiff(arrays[0], ...arrays.slice(1)));
          }
        };
      },
    ).args({ size: arraySizes, count: numberOfArrays });
  });
});

(async () => {
  await run();
})();
