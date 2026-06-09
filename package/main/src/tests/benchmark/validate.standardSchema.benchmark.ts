import { sValidator } from "@hono/standard-validator";
import { Hono } from "hono";
import { bench, run, summary, do_not_optimize } from "mitata";
import {
  array as vbArray,
  boolean as vbBoolean,
  number as vbNumber,
  object as vbObject,
  string as vbString,
} from "valibot";
import { z } from "zod";
import { arrayOf } from "@/Validate/array";
import { boolean } from "@/Validate/boolean";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object";
import { string } from "@/Validate/string";

// UMT, Zod, and Valibot all implement Standard Schema, so the same object can
// be driven both through the raw `~standard.validate` entry point and through
// Hono's `sValidator`, which consumes any Standard Schema. The direct group
// repeats the call enough times to make one sample last a few hundred
// milliseconds. The Hono group issues one real request per measured iteration
// with a hundred-element array payload so validation is a meaningful share of
// the request rather than being dwarfed by routing and JSON parsing.
const DIRECT_ITERATIONS = 500_000;

const umtSchema = object({ name: string(), age: number(), active: boolean() });
const zodSchema = z.object({
  name: z.string(),
  age: z.number(),
  active: z.boolean(),
});
const valibotSchema = vbObject({
  name: vbString(),
  age: vbNumber(),
  active: vbBoolean(),
});
const inputValue = { name: "John", age: 30, active: true };

// ------------------------------------------- raw Standard Schema interface
summary(() => {
  bench("standard · UMT", () => {
    for (let i = 0; i < DIRECT_ITERATIONS; i++) {
      do_not_optimize(umtSchema["~standard"].validate(inputValue));
    }
  });

  bench("standard · Zod", () => {
    for (let i = 0; i < DIRECT_ITERATIONS; i++) {
      do_not_optimize(zodSchema["~standard"].validate(inputValue));
    }
  });

  bench("standard · Valibot", () => {
    for (let i = 0; i < DIRECT_ITERATIONS; i++) {
      do_not_optimize(valibotSchema["~standard"].validate(inputValue));
    }
  });
});

// ------------------------------------------- Hono sValidator end-to-end
const umtListSchema = arrayOf(umtSchema);
const zodListSchema = z.array(zodSchema);
const valibotListSchema = vbArray(valibotSchema);
const umtApp = new Hono().post("/u", sValidator("json", umtListSchema), (c) =>
  c.json({ count: c.req.valid("json").length }),
);
const zodApp = new Hono().post("/u", sValidator("json", zodListSchema), (c) =>
  c.json({ count: c.req.valid("json").length }),
);
const valibotApp = new Hono().post(
  "/u",
  sValidator("json", valibotListSchema),
  (c) => c.json({ count: c.req.valid("json").length }),
);
const requestInit = {
  method: "POST",
  headers: { "content-type": "application/json" },
  body: JSON.stringify(Array.from({ length: 100 }, () => inputValue)),
};

summary(() => {
  bench("hono · UMT", async () => {
    do_not_optimize(await umtApp.request("/u", requestInit));
  });

  bench("hono · Zod", async () => {
    do_not_optimize(await zodApp.request("/u", requestInit));
  });

  bench("hono · Valibot", async () => {
    do_not_optimize(await valibotApp.request("/u", requestInit));
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
