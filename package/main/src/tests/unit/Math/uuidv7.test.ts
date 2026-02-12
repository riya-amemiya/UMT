import { uuidv7 } from "@/Math/uuidv7";

describe("uuidv7 function", () => {
  it("should generate UUID v7 with correct format", () => {
    const uuid = uuidv7();
    const uuidRegex =
      /^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/;
    expect(uuid).toMatch(uuidRegex);
  });

  it("should generate unique UUIDs", () => {
    const uuid1 = uuidv7();
    const uuid2 = uuidv7();
    expect(uuid1).not.toBe(uuid2);
  });

  it("should have version 7", () => {
    const uuid = uuidv7();
    const version = uuid.split("-")[2][0];
    expect(version).toBe("7");
  });

  it("should have correct variant", () => {
    const uuid = uuidv7();
    const variant = uuid.split("-")[3][0];
    expect(["8", "9", "a", "b"]).toContain(variant);
  });

  it("should generate non-colliding UUIDs", () => {
    const uuids = new Set<string>();
    for (let index = 0; index < 1000; index++) {
      const uuid = uuidv7();
      expect(uuids.has(uuid)).toBe(false);
      uuids.add(uuid);
    }
  });
});
