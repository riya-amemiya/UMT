import { number as vNumber } from "valibot";
import { number as zNumber, object as zObject, string as zString } from "zod";

import { Schema } from "@/Decorator/Schema";
import { validateInstance } from "@/Decorator/validateInstance";
import { number } from "@/Validate/number";
import type { StandardSchemaV1 } from "@/Validate/standardSchema";

const isValid = <Output>(
  schema: StandardSchemaV1<unknown, Output>,
  value: unknown,
): boolean => {
  const result = schema["~standard"].validate(value);
  if (result instanceof Promise) {
    return false;
  }
  return result.issues === undefined;
};

describe("Standard Schema interop", () => {
  describe("@Schema accepts external validators", () => {
    it("validates a field with a zod schema", () => {
      class Box {
        @Schema(zNumber()) count = 1;
      }

      expect(validateInstance(new Box()).type).toBe("success");
    });

    it("rejects an invalid field with a zod schema", () => {
      class Box {
        @Schema(zNumber()) count = "no" as unknown as number;
      }

      expect(validateInstance(new Box()).type).toBe("error");
    });

    it("validates a field with a valibot schema", () => {
      class Box {
        @Schema(vNumber()) count = 1;
      }

      expect(validateInstance(new Box()).type).toBe("success");
    });

    it("rejects an invalid field with a valibot schema", () => {
      class Box {
        @Schema(vNumber()) count = "no" as unknown as number;
      }

      expect(validateInstance(new Box()).type).toBe("error");
    });

    it("validates a nested object field with a zod schema", () => {
      class Form {
        @Schema(zObject({ id: zNumber(), name: zString() }))
        payload = { id: 1, name: "ok" };
      }

      expect(validateInstance(new Form()).type).toBe("success");
    });
  });

  describe("zod, valibot, and UMT validators share one interface", () => {
    it("accepts valid values across every implementation", () => {
      expect(isValid(zNumber(), 5)).toBe(true);
      expect(isValid(vNumber(), 5)).toBe(true);
      expect(isValid(number(), 5)).toBe(true);
    });

    it("rejects invalid values across every implementation", () => {
      expect(isValid(zNumber(), "x")).toBe(false);
      expect(isValid(vNumber(), "x")).toBe(false);
      expect(isValid(number(), "x")).toBe(false);
    });

    it("exposes a spec-compliant ~standard property on UMT validators", () => {
      const standard = number()["~standard"];

      expect(standard.version).toBe(1);
      expect(standard.vendor).toBe("umt");
      expect(typeof standard.validate).toBe("function");
    });
  });
});
