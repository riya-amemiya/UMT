import { unwrap } from "@/Tool/unwrap";

describe("unwrap", () => {
  it("should return the value when it is not undefined or null", () => {
    const value = "test";
    const result = unwrap(value, "Value is absent");
    expect(result).toBe("test");
  });

  it("should return the value when it is a number", () => {
    const value = 42;
    const result = unwrap(value, "Value is absent");
    expect(result).toBe(42);
  });

  it("should return the value when it is zero", () => {
    const value = 0;
    const result = unwrap(value, "Value is absent");
    expect(result).toBe(0);
  });

  it("should return the value when it is an empty string", () => {
    const value = "";
    const result = unwrap(value, "Value is absent");
    expect(result).toBe("");
  });

  it("should return the value when it is false", () => {
    const value = false;
    const result = unwrap(value, "Value is absent");
    expect(result).toBe(false);
  });

  it("should return the value when it is an object", () => {
    const value = { key: "value" };
    const result = unwrap(value, "Value is absent");
    expect(result).toEqual({ key: "value" });
  });

  it("should return the value when it is an array", () => {
    const value = [1, 2, 3];
    const result = unwrap(value, "Value is absent");
    expect(result).toEqual([1, 2, 3]);
  });

  it("should throw an error when the value is undefined", () => {
    const value = undefined;
    expect(() => unwrap(value, "Value is undefined")).toThrow(Error);
    expect(() => unwrap(value, "Value is undefined")).toThrow(
      "Value is undefined",
    );
  });

  it("should throw an error when the value is null", () => {
    // biome-ignore lint/suspicious/noEvolvingTypes: test
    const value = null;
    expect(() => unwrap(value, "Value is null")).toThrow(Error);
    expect(() => unwrap(value, "Value is null")).toThrow("Value is null");
  });
});
