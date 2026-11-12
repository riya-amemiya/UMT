import { buildUrl } from "@/URL/buildUrl";
import { parseQueryString } from "@/URL/parseQueryString";
import { stringifyQuery } from "@/URL/stringifyQuery";

describe("stringifyQuery", () => {
  test("serializes key-value pairs in insertion order", () => {
    expect(stringifyQuery({ page: "1", q: "search" })).toBe("page=1&q=search");
  });

  test("returns an empty string for an empty record", () => {
    expect(stringifyQuery({})).toBe("");
  });

  test("encodes as application/x-www-form-urlencoded", () => {
    expect(
      stringifyQuery({
        q: "hello world",
        t: "a&b=c",
        s: "~*-._!'()",
        u: "\u{3042}",
        e: "",
      }),
    ).toBe("q=hello+world&t=a%26b%3Dc&s=%7E*-._%21%27%28%29&u=%E3%81%82&e=");
  });

  test("encodes keys", () => {
    expect(stringifyQuery({ "a b": "1", "c&d": "2" })).toBe("a+b=1&c%26d=2");
  });

  test("matches the query string produced by buildUrl", () => {
    const parameters = { q: "hello world", tag: "a&b", emoji: "\u{1F600}" };
    expect(`https://example.com/?${stringifyQuery(parameters)}`).toBe(
      buildUrl("https://example.com", parameters),
    );
  });

  test("is the inverse of parseQueryString", () => {
    const parameters = { q: "hello world", tag: "a&b=c", plus: "1+1" };
    expect(parseQueryString(stringifyQuery(parameters))).toEqual(parameters);
  });
});
