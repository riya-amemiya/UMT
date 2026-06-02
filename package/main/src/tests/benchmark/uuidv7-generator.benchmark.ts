import { bench, run, summary, do_not_optimize } from "mitata";
import { uuidv7 } from "@/Math/uuidv7";

// Generating a single uuid takes only a few hundred nanoseconds. Repeating the
// generation this many times makes one measured sample take roughly ~500ms so
// that timer resolution and CPU scheduling jitter become negligible.
const ITERATIONS = 2_400_000;

summary(() => {
  bench("uuidv7 generator", () => {
    for (let i = 0; i < ITERATIONS; i++) {
      do_not_optimize(uuidv7());
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
