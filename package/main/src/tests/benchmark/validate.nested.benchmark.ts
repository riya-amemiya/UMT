import { bench, run, summary, do_not_optimize } from "mitata";
import {
  number as vbNumber,
  object as vbObject,
  safeParse as vbSafeParse,
  string as vbString,
} from "valibot";
import { z } from "zod";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object";
import { string } from "@/Validate/string";

// Validates a five-level-deep nested object so the comparison stresses
// recursive descent rather than a single flat shape. Each library builds the
// same tree of leaf and branch schemas. The loop repeats the validation enough
// times to bring one measured sample to a few hundred milliseconds so fixed
// overhead stays negligible. The count is shared across the libraries.
const NESTED_ITERATIONS = 200_000;

const umtLeaf = object({ a: string(), b: number() });
const umtLevel4 = object({ a: string(), b: number(), child: umtLeaf });
const umtLevel3 = object({ a: string(), b: number(), child: umtLevel4 });
const umtLevel2 = object({ a: string(), b: number(), child: umtLevel3 });
const umtNested = object({ a: string(), b: number(), child: umtLevel2 });

const zodLeaf = z.object({ a: z.string(), b: z.number() });
const zodLevel4 = z.object({ a: z.string(), b: z.number(), child: zodLeaf });
const zodLevel3 = z.object({ a: z.string(), b: z.number(), child: zodLevel4 });
const zodLevel2 = z.object({ a: z.string(), b: z.number(), child: zodLevel3 });
const zodNested = z.object({ a: z.string(), b: z.number(), child: zodLevel2 });

const valibotLeaf = vbObject({ a: vbString(), b: vbNumber() });
const valibotLevel4 = vbObject({
  a: vbString(),
  b: vbNumber(),
  child: valibotLeaf,
});
const valibotLevel3 = vbObject({
  a: vbString(),
  b: vbNumber(),
  child: valibotLevel4,
});
const valibotLevel2 = vbObject({
  a: vbString(),
  b: vbNumber(),
  child: valibotLevel3,
});
const valibotNested = vbObject({
  a: vbString(),
  b: vbNumber(),
  child: valibotLevel2,
});

const leaf = { a: "leaf", b: 5 };
const nestedValue = {
  a: "l1",
  b: 1,
  child: {
    a: "l2",
    b: 2,
    child: { a: "l3", b: 3, child: { a: "l4", b: 4, child: leaf } },
  },
};

summary(() => {
  bench("nested · UMT", () => {
    for (let i = 0; i < NESTED_ITERATIONS; i++) {
      do_not_optimize(umtNested(nestedValue));
    }
  });

  bench("nested · Zod", () => {
    for (let i = 0; i < NESTED_ITERATIONS; i++) {
      do_not_optimize(zodNested.safeParse(nestedValue));
    }
  });

  bench("nested · Valibot", () => {
    for (let i = 0; i < NESTED_ITERATIONS; i++) {
      do_not_optimize(vbSafeParse(valibotNested, nestedValue));
    }
  });
});

(async () => {
  try {
    await run();
    console.log("Benchmark finished successfully.");
  } catch (e) {
    console.error("Error during benchmark execution:", e);
  }
})();
