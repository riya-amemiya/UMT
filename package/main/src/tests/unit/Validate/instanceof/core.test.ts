import { instanceof_ } from "@/Validate/instanceof";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { optional } from "@/Validate/object/optional";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

class Animal {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}

class Dog extends Animal {
  bark() {
    return "woof";
  }
}

describe("instanceof_ validation", () => {
  it("accepts instances of the constructor", () => {
    const validator = instanceof_(Animal);
    expect(validator(new Animal("rex")).validate).toBe(true);
  });

  it("accepts instances of subclasses", () => {
    const validator = instanceof_(Animal);
    expect(validator(new Dog("rex")).validate).toBe(true);
  });

  it("rejects instances of unrelated constructors", () => {
    const validator = instanceof_(Dog);
    // @ts-expect-error Animal is not Dog
    expect(validator(new Animal("rex")).validate).toBe(false);
  });

  it("rejects non-objects", () => {
    const validator = instanceof_(Animal);
    // @ts-expect-error string is not an Animal
    expect(validator("rex").validate).toBe(false);
    // @ts-expect-error number is not an Animal
    expect(validator(1).validate).toBe(false);
    // @ts-expect-error null is not an Animal
    expect(validator(null).validate).toBe(false);
  });

  it("returns the configured message on failure", () => {
    const validator = instanceof_(Animal, "must be an Animal");
    // @ts-expect-error string is not an Animal
    expect(validator("rex").message).toBe("must be an Animal");
  });

  it("composes inside object()", () => {
    const validator = object({
      pet: instanceof_(Animal),
      name: string(),
    });
    const valid: ReturnType<typeof validator>["type"] = {
      pet: new Dog("rex"),
      name: "John",
    };
    expect(validator(valid).validate).toBe(true);
  });

  it("composes with union", () => {
    const validator = union(instanceof_(Animal), string());
    expect(validator(new Animal("rex")).validate).toBe(true);
    expect(validator("rex").validate).toBe(true);
    // @ts-expect-error number is not in the union
    expect(validator(42).validate).toBe(false);
  });

  it("composes with intersection of objects", () => {
    const validator = intersection(
      object({ pet: instanceof_(Animal) }),
      object({ id: number() }),
    );
    expect(validator({ pet: new Animal("rex"), id: 1 }).validate).toBe(true);
  });

  it("composes with nullable and optional", () => {
    const nullableAnimal = nullable(instanceof_(Animal));
    expect(nullableAnimal(null).validate).toBe(true);
    expect(nullableAnimal(new Animal("rex")).validate).toBe(true);

    const optionalAnimal = optional(instanceof_(Animal));
    expect(optionalAnimal(undefined).validate).toBe(true);
    expect(optionalAnimal(new Animal("rex")).validate).toBe(true);
  });

  it("infers the instance type via SchemaToInterface", () => {
    const validator = instanceof_(Animal);
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = new Animal("rex");
    expect(validator(value).validate).toBe(true);
  });
});
