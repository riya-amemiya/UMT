import { bench, run, summary, do_not_optimize } from "mitata";
import { uuidv7 } from "@/Math/uuidv7";

summary(() => {
  bench("uuidv7 generator", () => {
    do_not_optimize(uuidv7());
  });
});

(async () => {
  try {
    await run();
  } catch (e) {
    console.error(e);
  }
})();
