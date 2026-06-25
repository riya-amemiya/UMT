import { average, degToRad, gcd, lcm, range } from "../pkg/umt_plugin_wasm";

// Smoke tests for the auto-generated wrappers. These confirm that the codegen
// output is callable from JavaScript and forwards to umt_rust correctly.
describe("generated wrappers", () => {
  it("gcd computes the greatest common divisor", () => {
    expect(gcd(12, 18)).toBe(6);
    expect(gcd(7, 13)).toBe(1);
  });

  it("lcm computes the least common multiple", () => {
    expect(lcm(4, 6)).toBe(12);
  });

  it("degToRad converts degrees to radians", () => {
    expect(degToRad(180)).toBeCloseTo(Math.PI);
  });

  it("average averages a list of numbers", () => {
    expect(average([1, 2, 3, 4])).toBeCloseTo(2.5);
  });

  it("range builds an integer sequence", () => {
    expect(Array.from(range(0, 5))).toEqual([0, 1, 2, 3, 4]);
  });
});
