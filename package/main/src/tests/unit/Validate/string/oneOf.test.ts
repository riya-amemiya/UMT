import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { union } from "@/Validate/object/union";
import { oneOf, string } from "@/Validate/string";
import type { OneOfReturnType } from "@/Validate/string/oneOf";

type ExtractOneOfType<F> = F extends (value: string) => OneOfReturnType<infer T>
  ? T
  : never;

describe("oneOf", () => {
  it("validates a value included in the allowed list", () => {
    const validate = oneOf([
      "standard",
      "squat",
      "decanter",
      "round",
      "tall",
      "flask",
    ]);
    expect(validate("standard").validate).toBe(true);
    expect(validate("flask").validate).toBe(true);
    expect(validate("unknown").validate).toBe(false);
  });

  it("returns the custom message when validation fails", () => {
    const validate = oneOf(["a", "b"], "must be a or b");
    expect(validate("c").message).toBe("must be a or b");
    expect(validate("a").validate).toBe(true);
    expect(validate("a").message).toBe("");
  });

  it("returns an empty message when no message is provided", () => {
    const validate = oneOf(["x", "y"]);
    expect(validate("z").message).toBe("");
    expect(validate("z").validate).toBe(false);
  });

  it("rejects an empty allowed list", () => {
    const validate = oneOf([]);
    expect(validate("anything").validate).toBe(false);
  });

  it("treats the allowed list as case sensitive", () => {
    const validate = oneOf(["Tall"]);
    expect(validate("Tall").validate).toBe(true);
    expect(validate("tall").validate).toBe(false);
  });

  it("exposes the value through the type field for downstream inference", () => {
    const validate = oneOf(["a", "b"]);
    expect(validate("a").type).toBe("a");
    expect(validate("b").type).toBe("b");
  });

  it("preserves the literal union so it can be extracted via infer", () => {
    const validateBottleShape = oneOf([
      "standard",
      "squat",
      "decanter",
      "round",
      "tall",
      "flask",
    ]);
    type BottleShapeId = ExtractOneOfType<typeof validateBottleShape>;

    const accept = (id: BottleShapeId) => id;
    expect(accept("standard")).toBe("standard");
    expect(accept("flask")).toBe("flask");

    // @ts-expect-error - "wine" is not part of the union
    const reject: BottleShapeId = "wine";
    expect(reject).toBe("wine");
  });
});

describe("oneOf inside object()", () => {
  it("validates a property whose type is a string literal union", () => {
    const validateBottle = object({
      shape: oneOf(["standard", "squat", "decanter", "round", "tall", "flask"]),
      name: string(),
    });

    const valid: ReturnType<typeof validateBottle>["type"] = {
      shape: "standard",
      name: "wine bottle",
    };
    expect(validateBottle(valid).validate).toBe(true);

    const invalid = { shape: "unknown", name: "wine bottle" };
    // @ts-expect-error - "unknown" is not part of the shape union
    expect(validateBottle(invalid).validate).toBe(false);
  });

  it("propagates the failure message from oneOf when used in object()", () => {
    const validateBottle = object({
      shape: oneOf(["standard", "squat"], "invalid bottle shape"),
    });

    const invalid = { shape: "wine" };
    // @ts-expect-error - "wine" is not part of the shape union
    const result = validateBottle(invalid);
    expect(result.validate).toBe(false);
    expect(result.message).toBe("invalid bottle shape");
  });

  it("infers the literal union for the property type", () => {
    const validateBottle = object({
      shape: oneOf(["standard", "squat", "decanter", "round", "tall", "flask"]),
    });
    type Inferred = ReturnType<typeof validateBottle>["type"];

    const ok: Inferred = { shape: "standard" };
    expect(ok).toBeDefined();

    // @ts-expect-error - "wine" is not assignable to the shape union
    const ng: Inferred = { shape: "wine" };
    expect(ng).toBeDefined();
  });
});

describe("oneOf composed with union()", () => {
  it("accepts values matching the oneOf branch", () => {
    const validate = union(oneOf(["standard", "squat"]), number());
    expect(validate("standard").validate).toBe(true);
    expect(validate("squat").validate).toBe(true);
  });

  it("accepts values matching the other branch", () => {
    const validate = union(oneOf(["standard", "squat"]), number());
    expect(validate(42).validate).toBe(true);
  });

  it("rejects values matching neither branch", () => {
    const validate = union(oneOf(["standard", "squat"]), number());
    // @ts-expect-error - "wine" is not part of the union
    expect(validate("wine").validate).toBe(false);
  });

  it("merges the literal union into the validator's accepted parameter type", () => {
    const validate = union(oneOf(["standard", "squat"]), number());
    type Param = Parameters<typeof validate>[0];

    const fromOneOf: Param = "standard";
    const fromNumber: Param = 42;
    expect(fromOneOf).toBeDefined();
    expect(fromNumber).toBeDefined();

    // @ts-expect-error - "wine" is not part of the merged union
    const reject: Param = "wine";
    expect(reject).toBe("wine");
  });
});

describe("oneOf composed with intersection()", () => {
  it("works inside intersection() of objects", () => {
    const validate = intersection(
      object({
        shape: oneOf(["standard", "squat", "decanter"]),
      }),
      object({ size: number() }),
    );

    const valid: ReturnType<typeof validate>["type"] = {
      shape: "standard",
      size: 30,
    };
    expect(validate(valid).validate).toBe(true);

    const invalid = { shape: "wine", size: 30 };
    // @ts-expect-error - "wine" is not part of the shape union
    expect(validate(invalid).validate).toBe(false);
  });

  it("preserves the literal union for intersected object property", () => {
    const validate = intersection(
      object({ shape: oneOf(["standard", "squat"]) }),
      object({ size: number() }),
    );
    type Inferred = ReturnType<typeof validate>["type"];

    const ok: Inferred = {
      shape: "standard",
      size: 30,
    };
    expect(ok).toBeDefined();

    // @ts-expect-error - "wine" is not assignable to the shape union
    const ng: Inferred = { shape: "wine", size: 30 };
    expect(ng).toBeDefined();
  });
});
