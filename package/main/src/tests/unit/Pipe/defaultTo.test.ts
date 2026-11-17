import { defaultTo } from "@/Pipe/defaultTo";
import { pipe } from "@/Tool/pipe";

describe("defaultTo", () => {
  it("returns the default value for undefined", () => {
    expect(pipe(undefined, defaultTo(""))).toBe("");
  });

  it("returns the default value for null", () => {
    expect(pipe(null, defaultTo(0))).toBe(0);
  });

  it("returns the input when it is defined", () => {
    const value: string | undefined = "text";
    expect(pipe(value, defaultTo(""))).toBe("text");
  });

  it("keeps falsy values that are not null or undefined", () => {
    expect(pipe(0, defaultTo(1))).toBe(0);
    expect(pipe("", defaultTo("x"))).toBe("");
    expect(pipe(false, defaultTo(true))).toBe(false);
  });
});
