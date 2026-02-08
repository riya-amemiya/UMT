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

summary(() => {
  for (const size of arraySizes) {
    const arr = sharedArrays.get(size);
    if (!arr) {
      throw new Error(`No shared array found for size: ${size}`);
    }

    bench(`UMT lazy pipeline (size: ${size})`, () => {
      const mapped = lazyMap(arr, double);
      const filtered = lazyFilter(mapped, isEven);
      const taken = lazyTake(filtered, takeCount);
      do_not_optimize([...taken]);
    });

    bench(`Array native pipeline (size: ${size})`, () => {
      do_not_optimize(arr.map(double).filter(isEven).slice(0, takeCount));
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
