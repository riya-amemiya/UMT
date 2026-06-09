import { bench, run, summary, do_not_optimize } from "mitata";
import {
  array as vbArray,
  minValue as vbMinValue,
  number as vbNumber,
  object as vbObject,
  optional as vbOptional,
  picklist as vbPicklist,
  pipe as vbPipe,
  safeParse as vbSafeParse,
  string as vbString,
  union as vbUnion,
} from "valibot";
import { z } from "zod";
import { arrayOf } from "@/Validate/array";
import { number } from "@/Validate/number";
import { minValue } from "@/Validate/number/minValue";
import { object } from "@/Validate/object";
import { optional } from "@/Validate/object/optional";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import { oneOf } from "@/Validate/string/oneOf";

// Exercises paths that a flat happy-path benchmark misses: fail-fast rejection,
// union branch selection, optional members left out, literal-union membership,
// and a large primitive array. Each group repeats its validation enough times
// to bring one measured sample to a few hundred milliseconds so fixed overhead
// stays negligible. The count is shared across the libraries within a group.
const FAILURE_ITERATIONS = 1_000_000;
const UNION_ITERATIONS = 1_000_000;
const OPTIONAL_ITERATIONS = 500_000;
const ENUM_ITERATIONS = 1_000_000;
const LARGE_ARRAY_ITERATIONS = 2000;

// ------------------------------------------- fail fast (first key is wrong)
const umtFailure = object({ name: string(), age: number() });
const zodFailure = z.object({ name: z.string(), age: z.number() });
const valibotFailure = vbObject({ name: vbString(), age: vbNumber() });
const failureValue = { name: 123, age: 30 };

summary(() => {
  bench("failure · UMT", () => {
    for (let i = 0; i < FAILURE_ITERATIONS; i++) {
      do_not_optimize(umtFailure(failureValue as never));
    }
  });

  bench("failure · Zod", () => {
    for (let i = 0; i < FAILURE_ITERATIONS; i++) {
      do_not_optimize(zodFailure.safeParse(failureValue));
    }
  });

  bench("failure · Valibot", () => {
    for (let i = 0; i < FAILURE_ITERATIONS; i++) {
      do_not_optimize(vbSafeParse(valibotFailure, failureValue));
    }
  });
});

// ------------------------------------------- union (matches second branch)
const umtUnion = union(string(), number());
const zodUnion = z.union([z.string(), z.number()]);
const valibotUnion = vbUnion([vbString(), vbNumber()]);
const unionValue = 42;

summary(() => {
  bench("union · UMT", () => {
    for (let i = 0; i < UNION_ITERATIONS; i++) {
      do_not_optimize(umtUnion(unionValue));
    }
  });

  bench("union · Zod", () => {
    for (let i = 0; i < UNION_ITERATIONS; i++) {
      do_not_optimize(zodUnion.safeParse(unionValue));
    }
  });

  bench("union · Valibot", () => {
    for (let i = 0; i < UNION_ITERATIONS; i++) {
      do_not_optimize(vbSafeParse(valibotUnion, unionValue));
    }
  });
});

// ------------------------------------------- optional members left out
const umtOptional = object({
  id: string(),
  nick: optional(string()),
  bio: optional(string()),
});
const zodOptional = z.object({
  id: z.string(),
  nick: z.string().optional(),
  bio: z.string().optional(),
});
const valibotOptional = vbObject({
  id: vbString(),
  nick: vbOptional(vbString()),
  bio: vbOptional(vbString()),
});
const optionalValue = { id: "u1" };

summary(() => {
  bench("optional · UMT", () => {
    for (let i = 0; i < OPTIONAL_ITERATIONS; i++) {
      do_not_optimize(umtOptional(optionalValue as never));
    }
  });

  bench("optional · Zod", () => {
    for (let i = 0; i < OPTIONAL_ITERATIONS; i++) {
      do_not_optimize(zodOptional.safeParse(optionalValue));
    }
  });

  bench("optional · Valibot", () => {
    for (let i = 0; i < OPTIONAL_ITERATIONS; i++) {
      do_not_optimize(vbSafeParse(valibotOptional, optionalValue));
    }
  });
});

// ------------------------------------------- literal union membership
const enumValues = ["a", "b", "c", "d", "e", "f"] as const;
const umtEnum = oneOf(enumValues);
const zodEnum = z.enum(enumValues);
const valibotEnum = vbPicklist(enumValues);
const enumValue = "f";

summary(() => {
  bench("enum · UMT", () => {
    for (let i = 0; i < ENUM_ITERATIONS; i++) {
      do_not_optimize(umtEnum(enumValue));
    }
  });

  bench("enum · Zod", () => {
    for (let i = 0; i < ENUM_ITERATIONS; i++) {
      do_not_optimize(zodEnum.safeParse(enumValue));
    }
  });

  bench("enum · Valibot", () => {
    for (let i = 0; i < ENUM_ITERATIONS; i++) {
      do_not_optimize(vbSafeParse(valibotEnum, enumValue));
    }
  });
});

// ------------------------------------------- large primitive array
const umtLargeArray = arrayOf(number([minValue(0)]));
const zodLargeArray = z.array(z.number().min(0));
const valibotLargeArray = vbArray(vbPipe(vbNumber(), vbMinValue(0)));
const largeArrayValue = Array.from({ length: 1000 }, (_, i) => i);

summary(() => {
  bench("largeArray · UMT", () => {
    for (let i = 0; i < LARGE_ARRAY_ITERATIONS; i++) {
      do_not_optimize(umtLargeArray(largeArrayValue));
    }
  });

  bench("largeArray · Zod", () => {
    for (let i = 0; i < LARGE_ARRAY_ITERATIONS; i++) {
      do_not_optimize(zodLargeArray.safeParse(largeArrayValue));
    }
  });

  bench("largeArray · Valibot", () => {
    for (let i = 0; i < LARGE_ARRAY_ITERATIONS; i++) {
      do_not_optimize(vbSafeParse(valibotLargeArray, largeArrayValue));
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
