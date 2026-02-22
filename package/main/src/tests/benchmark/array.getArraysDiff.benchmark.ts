import { run, bench, summary, lineplot, do_not_optimize } from "mitata";
import { getArraysDiff } from "../../Array/getArraysDiff";

const arraySizes = [100, 1000, 10_000];
const numberOfArrays = [2, 5];

summary(() => {
  lineplot(() => {
    bench("getArraysDiff(size: $size, arrays: $count)", function* (state) {
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

      yield {
        0() {
          return arrays;
        },
        bench() {
          do_not_optimize(getArraysDiff(arrays[0], ...arrays.slice(1)));
        },
      };
    }).args({ size: arraySizes, count: numberOfArrays });
  });
});

(async () => {
  await run();
})();
