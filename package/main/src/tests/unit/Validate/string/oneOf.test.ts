import { oneOf } from "@/Validate/string";
import type { ValidateCoreReturnType } from "@/Validate/type";

type ExtractOneOfType<F> = F extends (
  value: string,
) => ValidateCoreReturnType<infer T>
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

  it("returns 'string' as the type tag", () => {
    const validate = oneOf(["a", "b"]);
    expect(validate("a").type).toBe("string");
    expect(validate("c").type).toBe("string");
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
