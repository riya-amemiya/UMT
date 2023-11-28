import { uuid } from "@/Validate/string/uuid";
import { string } from "@/Validate";

describe("uuid", () => {
  it("有効なUUIDを検証する", () => {
    const validateUUID = uuid();
    expect(validateUUID.validate("123e4567-e89b-42d3-a456-426614174000")).toBe(
      true,
    );
    expect(validateUUID.validate("3A86B1AB-237E-473D-A38F-06AA6A2A4783")).toBe(
      true,
    );
    expect(validateUUID.validate("B86D03E7-7B2D-4710-897B-77D05A1F0B4B")).toBe(
      true,
    );
    expect(validateUUID.validate("573F42D5-1734-45E1-9D4D-AA43E6DF697E")).toBe(
      true,
    );
    expect(validateUUID.validate("D1643B1F-DCEC-46A0-9196-D2C9B8446601")).toBe(
      true,
    );
    expect(validateUUID.validate("44046745-283C-4507-955E-430DAAD16189")).toBe(
      true,
    );
    expect(validateUUID.validate("7185AF69-0153-4C42-B63F-368291A74AAF")).toBe(
      true,
    );
    expect(validateUUID.validate("ADD5ABC1-54B8-4D5D-A8A0-ABA55089A1CF")).toBe(
      true,
    );
    expect(validateUUID.validate("0C338BD0-768B-4510-95D0-0D1571B08405")).toBe(
      true,
    );
    expect(validateUUID.validate("C48EACCC-CEA3-4C4D-B669-0D1669D4DBBF")).toBe(
      true,
    );
    expect(validateUUID.validate("A95E3F62-2D0E-4257-95D3-B239AF08115D")).toBe(
      true,
    );
    expect(validateUUID.validate("A95E3F622D0E425795D3B239AF08115D")).toBe(
      true,
    );
  });

  it("無効なUUIDを検証する", () => {
    const validateUUID = uuid();
    expect(validateUUID.validate("123e4567-e89b-42d3-a456-42661417400Z")).toBe(
      false,
    );
    expect(validateUUID.validate("123e4567-e89b-92d3-a456-426614174000")).toBe(
      false,
    );
  });

  it("カスタムメッセージを持つ有効なUUIDを検証する", () => {
    const validUUID = "123e4567-e89b-42d3-a456-426614174000";
    const customMessage = "UUIDが無効です";
    const validateUUID = string([uuid([4], customMessage)]);
    expect(validateUUID(validUUID).validate).toBe(true);
    expect(validateUUID(validUUID).message).toBe("");
  });

  it("カスタムメッセージを持つ無効なUUIDを検証する", () => {
    const invalidUUID = "123e4567-e89b-12d3-a456-42661417400Z";
    const customMessage = "UUIDが無効です";
    const validateUUID = string([uuid([4], customMessage)]);
    expect(validateUUID(invalidUUID).validate).toBe(false);
    expect(validateUUID(invalidUUID).message).toBe(customMessage);
  });

  it("大文字と小文字が混在する有効なUUIDを検証する", () => {
    const validateUUID = uuid();
    expect(validateUUID.validate("123e4567-E89b-42D3-a456-426614174000")).toBe(
      true,
    );
  });

  it("UUIDのバージョンが異なる場合を検証する", () => {
    const validateUUID = uuid([1, 2, 3, 4, 5]);
    expect(validateUUID.validate("123e4567-e89b-12d3-a456-426614174000")).toBe(
      true,
    );
    expect(validateUUID.validate("123e4567-e89b-22d3-a456-426614174000")).toBe(
      true,
    );
    expect(validateUUID.validate("123e4567-e89b-32d3-a456-426614174000")).toBe(
      true,
    );
    expect(validateUUID.validate("123e4567-e89b-52d3-a456-426614174000")).toBe(
      true,
    );
  });

  it("UUIDの形式が完全に異なる文字列を検証する", () => {
    const validateUUID = uuid();
    expect(validateUUID.validate("completely-invalid-format")).toBe(false);
  });

  it("空文字列を検証する", () => {
    const validateUUID = uuid();
    expect(validateUUID.validate("")).toBe(false);
  });

  it("nullを検証する", () => {
    const validateUUID = uuid();
    // @ts-ignore
    expect(validateUUID.validate(null)).toBe(false);
  });

  it("UUIDの長さが不足している場合を検証する", () => {
    const validateUUID = uuid();
    expect(validateUUID.validate("123e4567-e89b-42d3-a456-42661417400")).toBe(
      false,
    );
  });

  it("UUIDの長さが超過している場合を検証する", () => {
    const validateUUID = uuid();
    expect(
      validateUUID.validate("123e4567-e89b-42d3-a456-4266141740001234"),
    ).toBe(false);
  });
});
