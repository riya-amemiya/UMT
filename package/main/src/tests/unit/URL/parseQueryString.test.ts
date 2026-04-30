import { removePrototype } from "@/Object/removePrototype";
import { parseQueryString } from "@/URL/parseQueryString";

describe("parseQueryString", () => {
  it("should parse a query string with leading ?", () => {
    const result = parseQueryString("?page=1&q=search");
    expect(result).toEqual({ page: "1", q: "search" });
  });

  it("should parse a query string without leading ?", () => {
    const result = parseQueryString("foo=bar&baz=qux");
    expect(result).toEqual({ foo: "bar", baz: "qux" });
  });

  it("should parse a full URL", () => {
    const result = parseQueryString("https://example.com?a=1&b=2");
    expect(result).toEqual({ a: "1", b: "2" });
  });

  it("should return empty object for empty string", () => {
    const result = parseQueryString("");
    expect(result).toEqual({});
  });

  it("should return empty object for ? only", () => {
    const result = parseQueryString("?");
    expect(result).toEqual({});
  });

  it("should handle encoded values", () => {
    const result = parseQueryString("?q=hello%20world");
    expect(result).toEqual({ q: "hello world" });
  });

  it("should handle a single parameter", () => {
    const result = parseQueryString("?key=value");
    expect(result).toEqual({ key: "value" });
  });

  it("should handle URL with no query string", () => {
    const result = parseQueryString("https://example.com/path");
    expect(result).toEqual({});
  });

  it("should not pollute global Object.prototype when query contains __proto__", () => {
    parseQueryString("?__proto__=polluted&safe=value");

    const clean = {} as Record<string, unknown>;
    expect("polluted" in clean).toBe(false);
  });

  it("should sanitize correctly with removePrototype", () => {
    const result = removePrototype(
      parseQueryString(
        "?__proto__=evil&constructor=evil&prototype=evil&ok=good",
      ),
    );

    expect(result).toEqual({ ok: "good" });
    expect(Object.hasOwn(result, "__proto__")).toBe(false);
    expect(Object.hasOwn(result, "constructor")).toBe(false);
    expect(Object.hasOwn(result, "prototype")).toBe(false);
  });
});
