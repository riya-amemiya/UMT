import type { Formatter } from "$/string/formatString/formatter";

describe("Formatter", () => {
  it("Formatter without arguments", () => {
    const _: Formatter = (value) => String(value).toUpperCase();
    expect(_("a")).toBe("A");
  });
  it("Formatter with arguments", () => {
    const _: Formatter = (value, ...arguments_) =>
      `${String(value)}${arguments_.join("")}`;
    expect(_("a", "b", "c")).toBe("abc");
  });
});
