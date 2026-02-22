import { bench, run, summary, do_not_optimize } from "mitata";
import { pickDeep } from "@/Object/pickDeep";

const largeObj: any = {};
for (let i = 0; i < 1000; i++) {
  largeObj[`key${i}`] = i;
}
largeObj.nested = { a: { b: { c: 1 } } };

const keys = [] as any[];
for (let i = 0; i < 100; i++) {
  keys.push(`key${i}`);
}
keys.push("nested.a.b.c");

summary(() => {
  bench("pickDeep", () => {
    do_not_optimize(pickDeep(largeObj, ...(keys as any)));
  });
});

(async () => {
  await run();
})();
