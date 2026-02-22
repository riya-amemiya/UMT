import { bench, run, summary, do_not_optimize } from "mitata";
import { addition } from "@/Math/addition";

const integers = Array.from({ length: 100 }, (_, i) => i);
const floats = Array.from({ length: 100 }, (_, i) => i * 0.1);
const mixed = [...integers, ...floats];

summary(() => {
  bench("addition (100 integers)", () => {
    do_not_optimize(addition(...integers));
  });

  bench("addition (100 floats)", () => {
    do_not_optimize(addition(...floats));
  });

  bench("addition (200 mixed)", () => {
    do_not_optimize(addition(...mixed));
  });

  bench("addition (2 floats: 0.1 + 0.2)", () => {
    do_not_optimize(addition(0.1, 0.2));
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
