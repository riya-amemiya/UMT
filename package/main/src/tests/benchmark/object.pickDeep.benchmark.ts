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

summary(() => {
  bench("pickDeep", () => {
    // biome-ignore lint/suspicious/noExplicitAny: benchmark keys
    const k = keys as any;
    do_not_optimize(pickDeep(largeObj, ...k));
  });
});

(async () => {
  await run();
})();
