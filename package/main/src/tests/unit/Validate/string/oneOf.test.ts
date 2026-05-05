import { oneOf, string } from "@/Validate/string";

describe("oneOf", () => {
  it("validates a value included in the allowed list", () => {
    const bottleShapes = [
      "standard",
      "squat",
      "decanter",
      "round",
      "tall",
      "flask",
    ] as const;
    const validator = string([oneOf(bottleShapes)]);
    expect(validator("standard").validate).toBe(true);
    expect(validator("flask").validate).toBe(true);
    expect(validator("unknown").validate).toBe(false);
  });

  it("returns the custom message when validation fails", () => {
    const validator = string([oneOf(["a", "b"], "must be a or b")]);
    expect(validator("c").message).toBe("must be a or b");
    expect(validator("a").validate).toBe(true);
  });

  it("returns an empty message when no message is provided", () => {
    const validator = string([oneOf(["x", "y"])]);
    expect(validator("z").message).toBe("");
    expect(validator("z").validate).toBe(false);
  });

  it("rejects an empty allowed list", () => {
    const validator = string([oneOf([])]);
    expect(validator("anything").validate).toBe(false);
  });

  it("treats the allowed list as case sensitive", () => {
    const validator = string([oneOf(["Tall"])]);
    expect(validator("Tall").validate).toBe(true);
    expect(validator("tall").validate).toBe(false);
  });
});
