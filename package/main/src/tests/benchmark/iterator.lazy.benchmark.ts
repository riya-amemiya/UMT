import { run, bench, summary, do_not_optimize } from "mitata";
import { lazyMap } from "@/Iterator/lazyMap";
import { lazyFilter } from "@/Iterator/lazyFilter";
import { lazyTake } from "@/Iterator/lazyTake";

const arraySizes = [10_000, 100_000, 1_000_000];

const sharedArrays = new Map<number, number[]>();
for (const size of arraySizes) {
  sharedArrays.set(
    size,
    Array.from({ length: size }, (_, i) => i),
  );
}

const double = (n: number) => n * 2;
const isEven = (n: number) => n % 2 === 0;
const takeCount = 100;

// The lazy pipeline short-circuits after taking 100 elements, so its cost is a
// few microseconds regardless of the source size; it therefore uses a fixed
// repetition count. The eager native pipeline scans the whole array, so its
// cost grows with the size and uses a size-based count. Both are sized so one
// measured sample reaches a few milliseconds and stays above CI-runner jitter,
// and the counts are fixed so the base/head comparison stays meaningful.
const LAZY_ITERATIONS = 1000;
const nativeIterations = (size: number): number => {
  if (size <= 10_000) {
    return 50;
  }
  if (size <= 100_000) {
    return 7;
  }
  return 1;
};

summary(() => {
  for (const size of arraySizes) {
    const arr = sharedArrays.get(size);
    if (!arr) {
      throw new Error(`No shared array found for size: ${size}`);
    }
    const nativeReps = nativeIterations(size);

    bench(`UMT lazy pipeline (size: ${size})`, () => {
      for (let i = 0; i < LAZY_ITERATIONS; i++) {
        const mapped = lazyMap(arr, double);
        const filtered = lazyFilter(mapped, isEven);
        const taken = lazyTake(filtered, takeCount);
        do_not_optimize([...taken]);
      }
    });

    bench(`Array native pipeline (size: ${size})`, () => {
      for (let i = 0; i < nativeReps; i++) {
        do_not_optimize(arr.map(double).filter(isEven).slice(0, takeCount));
      }
    });
  }
});

(async () => {
  try {
    await run();
  } catch (e) {
    console.error(e);
  }
})();
