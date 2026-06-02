import { bench, run, summary, do_not_optimize } from "mitata";
import { randomSelect } from "@/Array/randomSelect";
import { shuffle } from "@/Array/shuffle";

const size = 10_000;
const array = Array.from({ length: size }, (_, i) => i);

// A single selection runs in tens to hundreds of microseconds, which is buried
// in CI-runner jitter. Repeating the operation a fixed number of times lifts
// one measured sample to a few milliseconds so it stays above that jitter. The
// same count is shared across all cases so the comparison stays fair.
const ITERATIONS = 30;

summary(() => {
  bench(`randomSelect(size=${size}, count=${size * 0.1})`, () => {
    for (let i = 0; i < ITERATIONS; i++) {
      do_not_optimize(randomSelect(array, size * 0.1));
    }
  });

  bench(`randomSelect(size=${size}, count=${size * 0.5})`, () => {
    for (let i = 0; i < ITERATIONS; i++) {
      do_not_optimize(randomSelect(array, size * 0.5));
    }
  });

  bench(
    `randomSelect(size=${size}, count=${size * 0.8}) (optimization threshold)`,
    () => {
      for (let i = 0; i < ITERATIONS; i++) {
        do_not_optimize(randomSelect(array, size * 0.8));
      }
    },
  );

  bench(`randomSelect(size=${size}, count=${size * 0.9})`, () => {
    for (let i = 0; i < ITERATIONS; i++) {
      do_not_optimize(randomSelect(array, size * 0.9));
    }
  });

  bench(`randomSelect(size=${size}, count=${size * 0.99})`, () => {
    for (let i = 0; i < ITERATIONS; i++) {
      do_not_optimize(randomSelect(array, size * 0.99));
    }
  });

  // Reference baseline: Full shuffle + truncate
  bench(`shuffle(size=${size}) + truncate to ${size * 0.9}`, () => {
    for (let i = 0; i < ITERATIONS; i++) {
      const res = shuffle(array);
      res.length = size * 0.9;
      do_not_optimize(res);
    }
  });
});

await run();
