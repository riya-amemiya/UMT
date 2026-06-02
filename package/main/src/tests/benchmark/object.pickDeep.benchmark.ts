import { bench, run, summary, do_not_optimize } from "mitata";
import { pickDeep } from "@/Object/pickDeep";

// biome-ignore lint/suspicious/noExplicitAny: benchmark object
const largeObj: any = {};
for (let i = 0; i < 1000; i++) {
  largeObj[`key${i}`] = i;
}
largeObj.nested = { a: { b: { c: 1 } } };

const keys: string[] = [];
for (let i = 0; i < 100; i++) {
  keys.push(`key${i}`);
}
keys.push("nested.a.b.c");

// A single pickDeep call takes tens of microseconds, so repeating it this many
// times makes one measured sample take roughly ~500ms, keeping fixed
// measurement overhead negligible.
const ITERATIONS = 30_000;

summary(() => {
  bench("pickDeep", () => {
    // biome-ignore lint/suspicious/noExplicitAny: benchmark keys
    const k = keys as any;
    for (let i = 0; i < ITERATIONS; i++) {
      // biome-ignore lint/suspicious/noExplicitAny: benchmark call
      do_not_optimize((pickDeep as any)(largeObj, ...k));
    }
  });
});

(async () => {
  await run();
})();
