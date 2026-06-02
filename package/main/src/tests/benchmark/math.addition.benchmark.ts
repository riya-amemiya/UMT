import { bench, run, summary, do_not_optimize } from "mitata";
import { addition } from "@/Math/addition";

const integers = Array.from({ length: 100 }, (_, i) => i);
const floats = Array.from({ length: 100 }, (_, i) => i * 0.1);
const mixed = [...integers, ...floats];

// Each case has a very different cost (the two-float case is in the hundreds of
// nanoseconds while the larger inputs are tens of microseconds), so a per-case
// repetition count is used to bring every measured sample to roughly ~500ms.
// This keeps fixed measurement overhead negligible for every input shape.
const INTEGERS_ITERATIONS = 105_000;
const FLOATS_ITERATIONS = 31_000;
const MIXED_ITERATIONS = 21_000;
const TWO_FLOATS_ITERATIONS = 1_300_000;

summary(() => {
  bench("addition (100 integers)", () => {
    for (let i = 0; i < INTEGERS_ITERATIONS; i++) {
      do_not_optimize(addition(...integers));
    }
  });

  bench("addition (100 floats)", () => {
    for (let i = 0; i < FLOATS_ITERATIONS; i++) {
      do_not_optimize(addition(...floats));
    }
  });

  bench("addition (200 mixed)", () => {
    for (let i = 0; i < MIXED_ITERATIONS; i++) {
      do_not_optimize(addition(...mixed));
    }
  });

  bench("addition (2 floats: 0.1 + 0.2)", () => {
    for (let i = 0; i < TWO_FLOATS_ITERATIONS; i++) {
      do_not_optimize(addition(0.1, 0.2));
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
