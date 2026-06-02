import { run, bench, summary, do_not_optimize } from "mitata";
import { every } from "@/Predicate/every";
import { some } from "@/Predicate/some";
import { not } from "@/Predicate/not";

const isPositive = (n: number) => n > 0;
const isEven = (n: number) => n % 2 === 0;
const isLessThan1000 = (n: number) => n < 1000;

const umtEvery = every(isPositive, isEven, isLessThan1000);
const umtSome = some(isPositive, isEven, isLessThan1000);
const umtNot = not(isPositive);

const inlineEvery = (n: number) => n > 0 && n % 2 === 0 && n < 1000;
const inlineSome = (n: number) => n > 0 || n % 2 === 0 || n < 1000;
const inlineNot = (n: number) => !(n > 0);

const testValues = Array.from({ length: 10_000 }, (_, i) => i - 5000);

// One pass over the 10k values already runs in tens to a hundred microseconds,
// but that is still noise-dominated. Each group repeats the pass enough times
// for one measured sample to take roughly ~500ms, keeping fixed overhead
// negligible. The count is shared between the composed and inline variants.
const EVERY_ITERATIONS = 4500;
const SOME_ITERATIONS = 4500;
const NOT_ITERATIONS = 38_000;

summary(() => {
  bench("UMT every composition", () => {
    for (let i = 0; i < EVERY_ITERATIONS; i++) {
      for (const v of testValues) {
        do_not_optimize(umtEvery(v));
      }
    }
  });

  bench("Inline arrow every", () => {
    for (let i = 0; i < EVERY_ITERATIONS; i++) {
      for (const v of testValues) {
        do_not_optimize(inlineEvery(v));
      }
    }
  });
});

summary(() => {
  bench("UMT some composition", () => {
    for (let i = 0; i < SOME_ITERATIONS; i++) {
      for (const v of testValues) {
        do_not_optimize(umtSome(v));
      }
    }
  });

  bench("Inline arrow some", () => {
    for (let i = 0; i < SOME_ITERATIONS; i++) {
      for (const v of testValues) {
        do_not_optimize(inlineSome(v));
      }
    }
  });
});

summary(() => {
  bench("UMT not composition", () => {
    for (let i = 0; i < NOT_ITERATIONS; i++) {
      for (const v of testValues) {
        do_not_optimize(umtNot(v));
      }
    }
  });

  bench("Inline arrow not", () => {
    for (let i = 0; i < NOT_ITERATIONS; i++) {
      for (const v of testValues) {
        do_not_optimize(inlineNot(v));
      }
    }
  });
});

(async () => {
  try {
    await run();
  } catch (e) {
    console.error(e);
  }
})();
