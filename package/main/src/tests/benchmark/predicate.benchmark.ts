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

summary(() => {
  bench("UMT every composition", () => {
    for (const v of testValues) {
      do_not_optimize(umtEvery(v));
    }
  });

  bench("Inline arrow every", () => {
    for (const v of testValues) {
      do_not_optimize(inlineEvery(v));
    }
  });
});

summary(() => {
  bench("UMT some composition", () => {
    for (const v of testValues) {
      do_not_optimize(umtSome(v));
    }
  });

  bench("Inline arrow some", () => {
    for (const v of testValues) {
      do_not_optimize(inlineSome(v));
    }
  });
});

summary(() => {
  bench("UMT not composition", () => {
    for (const v of testValues) {
      do_not_optimize(umtNot(v));
    }
  });

  bench("Inline arrow not", () => {
    for (const v of testValues) {
      do_not_optimize(inlineNot(v));
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
