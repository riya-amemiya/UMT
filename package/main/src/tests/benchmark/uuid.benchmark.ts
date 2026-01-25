import { bench, run, summary, do_not_optimize } from "mitata";
import { uuid } from "@/Validate/string/uuid";

const uuidV4 = "123e4567-e89b-42d3-a456-426614174000";
const invalidUUID = "123e4567-e89b-42d3-a456-42661417400Z";

const validatorV4 = uuid([4]);
const validatorAll = uuid([1, 2, 3, 4, 5]);

summary(() => {
  bench("validate UUID v4 (single version)", () => {
    do_not_optimize(validatorV4.validate(uuidV4));
  });

  bench("validate UUID invalid (single version)", () => {
    do_not_optimize(validatorV4.validate(invalidUUID));
  });

  bench("validate UUID v4 (all versions)", () => {
    do_not_optimize(validatorAll.validate(uuidV4));
  });
});

(async () => {
  try {
    await run();
  } catch (e) {
    console.error(e);
  }
})();
