import { IsNumber } from "@/Decorator/IsNumber";
import { IsString } from "@/Decorator/IsString";
import { Validatable } from "@/Decorator/Validatable";

describe("Validatable", () => {
  it("constructs normally when every field is valid", () => {
    @Validatable
    class Product {
      @IsNumber price = 100;
      @IsString name = "pen";
    }

    expect(() => new Product()).not.toThrow();
  });

  it("throws when a decorated field is invalid", () => {
    @Validatable
    class Product {
      @IsNumber price = "free" as unknown as number;
    }

    expect(() => new Product()).toThrow("price must be a number");
  });

  it("joins every failure message", () => {
    @Validatable
    class Product {
      @IsNumber price = "free" as unknown as number;
      @IsString name = 42 as unknown as string;
    }

    expect(() => new Product()).toThrow(
      "price must be a number; name must be a string",
    );
  });

  it("preserves constructor arguments and instance type", () => {
    @Validatable
    class Point {
      x: number;
      y: number;
      @IsNumber sum: number;

      constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
        this.sum = x + y;
      }
    }

    const point = new Point(2, 3);

    expect(point).toBeInstanceOf(Point);
    expect(point.sum).toBe(5);
  });
});
