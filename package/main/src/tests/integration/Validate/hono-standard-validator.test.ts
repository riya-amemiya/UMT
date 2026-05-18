import { sValidator } from "@hono/standard-validator";
import { Hono } from "hono";
import { arrayOf } from "@/Validate/array/arrayOf";
import { boolean } from "@/Validate/boolean";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { optional } from "@/Validate/object/optional";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";

/**
 * Regression tests covering UMT validators consumed through
 * `@hono/standard-validator`. The previous release advertised
 * `StandardSchemaV1<unknown, unknown>` for `object()`, which collapsed
 * `c.req.valid("json")` to `unknown` and produced `TS18046` at every
 * field access. These tests fail to compile if the inferred output type
 * regresses to `unknown`, and fail at runtime if the validator's
 * `~standard.validate` reports invalid input as valid.
 */
describe("Integration: UMT validators with @hono/standard-validator", () => {
  it("infers c.req.valid('json') as the object shape and round-trips a valid request", async () => {
    const UpsertProfileInputSchema = object({
      displayName: string(),
      bio: string(),
      isPublic: boolean(),
    });

    const app = new Hono().post(
      "/profile",
      sValidator("json", UpsertProfileInputSchema),
      (c) => {
        const input = c.req.valid("json");
        // The following annotations would fail if `c.req.valid("json")`
        // collapsed back to `unknown`, guarding against the previous
        // regression where object() advertised StandardSchemaV1<unknown, unknown>.
        const displayName: string = input.displayName;
        const bio: string = input.bio;
        const isPublic: boolean = input.isPublic;
        return c.json({ displayName, bio, isPublic });
      },
    );

    const response = await app.request("/profile", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        displayName: "yuta",
        bio: "engineer",
        isPublic: true,
      }),
    });

    expect(response.status).toBe(200);
    expect(await response.json()).toStrictEqual({
      displayName: "yuta",
      bio: "engineer",
      isPublic: true,
    });
  });

  it("rejects a request that violates the object shape with status 400", async () => {
    const UpsertProfileInputSchema = object({
      displayName: string(),
      isPublic: boolean(),
    });

    const app = new Hono().post(
      "/profile",
      sValidator("json", UpsertProfileInputSchema),
      (c) => c.json(c.req.valid("json")),
    );

    const response = await app.request("/profile", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ displayName: 42, isPublic: "yes" }),
    });

    expect(response.status).toBe(400);
  });

  it("preserves optional() keys as `?:` in the inferred output type", async () => {
    const Schema = object({
      id: string(),
      nickname: optional(string()),
    });

    const app = new Hono().post(
      "/optional",
      sValidator("json", Schema),
      (c) => {
        const input = c.req.valid("json");
        const id: string = input.id;
        const nickname: string | undefined = input.nickname;
        return c.json({ id, nickname: nickname ?? null });
      },
    );

    const withoutNickname = await app.request("/optional", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ id: "u1" }),
    });
    expect(withoutNickname.status).toBe(200);
    expect(await withoutNickname.json()).toStrictEqual({
      id: "u1",
      nickname: null,
    });

    const withNickname = await app.request("/optional", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ id: "u2", nickname: "yuta" }),
    });
    expect(withNickname.status).toBe(200);
    expect(await withNickname.json()).toStrictEqual({
      id: "u2",
      nickname: "yuta",
    });
  });

  it("flows nullable() into the inferred output type as `T | null`", async () => {
    const Schema = object({
      avatarUrl: nullable(string()),
    });

    const app = new Hono().post("/nullable", sValidator("json", Schema), (c) =>
      c.json(c.req.valid("json")),
    );

    const nullCase = await app.request("/nullable", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ avatarUrl: null }),
    });
    expect(nullCase.status).toBe(200);
    expect(await nullCase.json()).toStrictEqual({ avatarUrl: null });

    const stringCase = await app.request("/nullable", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ avatarUrl: "https://example.com/a.png" }),
    });
    expect(stringCase.status).toBe(200);
  });

  it("supports arrayOf, union, and intersection at top level", async () => {
    const ListSchema = arrayOf(object({ id: number(), label: string() }));
    const UnionSchema = union(string(), number());
    const IntersectionSchema = intersection(
      object({ id: string() }),
      object({ label: string() }),
    );

    const app = new Hono()
      .post("/list", sValidator("json", ListSchema), (c) =>
        c.json({ count: c.req.valid("json").length }),
      )
      .post("/either", sValidator("json", UnionSchema), (c) =>
        c.json({ value: c.req.valid("json") }),
      )
      .post("/both", sValidator("json", IntersectionSchema), (c) => {
        const input = c.req.valid("json");
        const id: string = input.id;
        const label: string = input.label;
        return c.json({ id, label });
      });

    const list = await app.request("/list", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify([{ id: 1, label: "a" }]),
    });
    expect(list.status).toBe(200);
    expect(await list.json()).toStrictEqual({ count: 1 });

    const either = await app.request("/either", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify("hello"),
    });
    expect(either.status).toBe(200);

    const both = await app.request("/both", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ id: "i1", label: "l1" }),
    });
    expect(both.status).toBe(200);
    expect(await both.json()).toStrictEqual({ id: "i1", label: "l1" });
  });
});
