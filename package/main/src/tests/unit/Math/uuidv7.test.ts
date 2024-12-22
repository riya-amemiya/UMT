import { uuidv7 } from "@/Math/uuidv7";

describe("uuidv7", () => {
  it("生成されたUUID v7が正しい形式である", () => {
    const uuid = uuidv7();
    const uuidRegex =
      /^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/;
    expect(uuid).toMatch(uuidRegex);
  });

  it("生成されたUUID v7が一意である", () => {
    const uuid1 = uuidv7();
    const uuid2 = uuidv7();
    expect(uuid1).not.toBe(uuid2);
  });

  it("生成されたUUID v7のバージョンが7である", () => {
    const uuid = uuidv7();
    const version = uuid.split("-")[2][0];
    expect(version).toBe("7");
  });

  it("生成されたUUID v7のバリアントが正しい形式である", () => {
    const uuid = uuidv7();
    const variant = uuid.split("-")[3][0];
    expect(["8", "9", "a", "b"]).toContain(variant);
  });

  it("衝突しないUUID v7が生成される", () => {
    const uuids = new Set<string>();
    for (let index = 0; index < 1000; index++) {
      const uuid = uuidv7();
      expect(uuids.has(uuid)).toBe(false);
      uuids.add(uuid);
    }
  });
});
