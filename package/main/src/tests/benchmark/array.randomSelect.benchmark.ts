import { bench, run, summary, do_not_optimize } from "mitata";
import { randomSelect } from "@/Array/randomSelect";
import { shuffle } from "@/Array/shuffle";

const size = 10_000;
const array = Array.from({ length: size }, (_, i) => i);

summary(() => {
  bench(`randomSelect(size=${size}, count=${size * 0.1})`, () => {
    do_not_optimize(randomSelect(array, size * 0.1));
  });

  bench(`randomSelect(size=${size}, count=${size * 0.5})`, () => {
    do_not_optimize(randomSelect(array, size * 0.5));
  });

  bench(`randomSelect(size=${size}, count=${size * 0.8}) (optimization threshold)`, () => {
    do_not_optimize(randomSelect(array, size * 0.8));
  });

  bench(`randomSelect(size=${size}, count=${size * 0.9})`, () => {
    do_not_optimize(randomSelect(array, size * 0.9));
  });

  bench(`randomSelect(size=${size}, count=${size * 0.99})`, () => {
    do_not_optimize(randomSelect(array, size * 0.99));
  });

  // Reference baseline: Full shuffle + truncate
  bench(`shuffle(size=${size}) + truncate to ${size * 0.9}`, () => {
    const res = shuffle(array);
    res.length = size * 0.9;
    do_not_optimize(res);
  });
});

await run();
