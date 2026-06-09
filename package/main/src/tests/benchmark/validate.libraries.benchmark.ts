import { bench, run, summary, do_not_optimize } from "mitata";
import {
  array as vbArray,
  boolean as vbBoolean,
  maxLength as vbMaxLength,
  maxValue as vbMaxValue,
  minLength as vbMinLength,
  minValue as vbMinValue,
  number as vbNumber,
  object as vbObject,
  pipe as vbPipe,
  safeParse as vbSafeParse,
  string as vbString,
} from "valibot";
import { z } from "zod";
import { arrayOf } from "@/Validate/array";
import { boolean } from "@/Validate/boolean";
import { number } from "@/Validate/number";
import { maxValue } from "@/Validate/number/maxValue";
import { minValue } from "@/Validate/number/minValue";
import { object } from "@/Validate/object";
import { string } from "@/Validate/string";
import { maxLength } from "@/Validate/string/maxLength";
import { minLength } from "@/Validate/string/minLength";

// Compares UMT validators against Zod and Valibot on representative shapes.
// UMT validators only report a pass/fail result, so the Zod and Valibot
// schemas are run through their safe-parse entry points to keep the compared
// work equivalent (validate without throwing). Each group repeats a single
// validation enough times to bring one measured sample to a few hundred
// milliseconds so fixed overhead stays negligible. The count is shared across
// the libraries within a group.
const STRING_ITERATIONS = 1_000_000;
const NUMBER_ITERATIONS = 1_000_000;
const OBJECT_ITERATIONS = 300_000;
const ARRAY_ITERATIONS = 30_000;

// ---------------------------------------------------------------- string
const umtString = string([minLength(3), maxLength(20)]);
const zodString = z.string().min(3).max(20);
const valibotString = vbPipe(vbString(), vbMinLength(3), vbMaxLength(20));
const stringValue = "hello world";

summary(() => {
  bench("string · UMT", () => {
    for (let i = 0; i < STRING_ITERATIONS; i++) {
      do_not_optimize(umtString(stringValue));
    }
  });

  bench("string · Zod", () => {
    for (let i = 0; i < STRING_ITERATIONS; i++) {
      do_not_optimize(zodString.safeParse(stringValue));
    }
  });

  bench("string · Valibot", () => {
    for (let i = 0; i < STRING_ITERATIONS; i++) {
      do_not_optimize(vbSafeParse(valibotString, stringValue));
    }
  });
});

// ---------------------------------------------------------------- number
const umtNumber = number([minValue(0), maxValue(100)]);
const zodNumber = z.number().min(0).max(100);
const valibotNumber = vbPipe(vbNumber(), vbMinValue(0), vbMaxValue(100));
const numberValue = 42;

summary(() => {
  bench("number · UMT", () => {
    for (let i = 0; i < NUMBER_ITERATIONS; i++) {
      do_not_optimize(umtNumber(numberValue));
    }
  });

  bench("number · Zod", () => {
    for (let i = 0; i < NUMBER_ITERATIONS; i++) {
      do_not_optimize(zodNumber.safeParse(numberValue));
    }
  });

  bench("number · Valibot", () => {
    for (let i = 0; i < NUMBER_ITERATIONS; i++) {
      do_not_optimize(vbSafeParse(valibotNumber, numberValue));
    }
  });
});

// ---------------------------------------------------------------- object
const umtObject = object({
  name: string([minLength(1)]),
  age: number([minValue(0)]),
  email: string([minLength(3)]),
  active: boolean(),
});
const zodObject = z.object({
  name: z.string().min(1),
  age: z.number().min(0),
  email: z.string().min(3),
  active: z.boolean(),
});
const valibotObject = vbObject({
  name: vbPipe(vbString(), vbMinLength(1)),
  age: vbPipe(vbNumber(), vbMinValue(0)),
  email: vbPipe(vbString(), vbMinLength(3)),
  active: vbBoolean(),
});
const objectValue = { name: "John", age: 30, email: "j@x.io", active: true };

summary(() => {
  bench("object · UMT", () => {
    for (let i = 0; i < OBJECT_ITERATIONS; i++) {
      do_not_optimize(umtObject(objectValue));
    }
  });

  bench("object · Zod", () => {
    for (let i = 0; i < OBJECT_ITERATIONS; i++) {
      do_not_optimize(zodObject.safeParse(objectValue));
    }
  });

  bench("object · Valibot", () => {
    for (let i = 0; i < OBJECT_ITERATIONS; i++) {
      do_not_optimize(vbSafeParse(valibotObject, objectValue));
    }
  });
});

// ---------------------------------------------- array of objects
const umtArray = arrayOf(umtObject);
const zodArray = z.array(zodObject);
const valibotArray = vbArray(valibotObject);
const arrayValue = Array.from({ length: 20 }, () => objectValue);

summary(() => {
  bench("array · UMT", () => {
    for (let i = 0; i < ARRAY_ITERATIONS; i++) {
      do_not_optimize(umtArray(arrayValue as never));
    }
  });

  bench("array · Zod", () => {
    for (let i = 0; i < ARRAY_ITERATIONS; i++) {
      do_not_optimize(zodArray.safeParse(arrayValue));
    }
  });

  bench("array · Valibot", () => {
    for (let i = 0; i < ARRAY_ITERATIONS; i++) {
      do_not_optimize(vbSafeParse(valibotArray, arrayValue));
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
