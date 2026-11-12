import { encodeBase64Url } from "@/String/encodeBase64Url";

describe("encodeBase64Url", () => {
  test("encodes without padding", () => {
    expect(encodeBase64Url("Hello World")).toBe("SGVsbG8gV29ybGQ");
    expect(encodeBase64Url("f")).toBe("Zg");
    expect(encodeBase64Url("fo")).toBe("Zm8");
    expect(encodeBase64Url("foo")).toBe("Zm9v");
  });

  test("uses - and _ instead of + and /", () => {
    expect(encodeBase64Url("\u{FF}\u{FE}")).toBe("w7_Dvg");
    expect(encodeBase64Url("\u{1F600}?>")).toBe("8J-YgD8-");
  });

  test("encodes an empty string", () => {
    expect(encodeBase64Url("")).toBe("");
  });
});
