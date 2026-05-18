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

describe("Integration: UMT validators with @hono/standard-validator", () => {
  it("types c.req.valid('json') from an object() schema by property", async () => {
    const Schema = object({
      name: string(),
      age: number(),
      active: boolean(),
    });
    const app = new Hono().post("/u", sValidator("json", Schema), (c) => {
      const input = c.req.valid("json");
      const name: string = input.name;
      const age: number = input.age;
      const active: boolean = input.active;
      return c.json({ name, age, active });
    });
    const res = await app.request("/u", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ name: "x", age: 1, active: true }),
    });
    expect(res.status).toBe(200);
    expect(await res.json()).toStrictEqual({ name: "x", age: 1, active: true });
  });

  it("returns 400 for an object() request with a wrong-typed property", async () => {
    const Schema = object({ name: string() });
    const app = new Hono().post("/u", sValidator("json", Schema), (c) =>
      c.json(c.req.valid("json")),
    );
    const res = await app.request("/u", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ name: 42 }),
    });
    expect(res.status).toBe(400);
  });

  it("types optional() as T | undefined in c.req.valid('json')", async () => {
    const Schema = object({ id: string(), nick: optional(string()) });
    const app = new Hono().post("/u", sValidator("json", Schema), (c) => {
      const input = c.req.valid("json");
      const id: string = input.id;
      const nick: string | undefined = input.nick;
      return c.json({ id, nick: nick ?? null });
    });
    const res = await app.request("/u", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ id: "u1" }),
    });
    expect(res.status).toBe(200);
    expect(await res.json()).toStrictEqual({ id: "u1", nick: null });
  });

  it("types nullable() as T | null in c.req.valid('json')", async () => {
    const Schema = object({ url: nullable(string()) });
    const app = new Hono().post("/u", sValidator("json", Schema), (c) => {
      const url: string | null = c.req.valid("json").url;
      return c.json({ url });
    });
    const res = await app.request("/u", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ url: null }),
    });
    expect(res.status).toBe(200);
    expect(await res.json()).toStrictEqual({ url: null });
  });

  it("types arrayOf(object()) elements in c.req.valid('json')", async () => {
    const Schema = arrayOf(object({ id: number(), label: string() }));
    const app = new Hono().post("/u", sValidator("json", Schema), (c) => {
      const list = c.req.valid("json");
      const firstId: number = list[0].id;
      const firstLabel: string = list[0].label;
      return c.json({ firstId, firstLabel });
    });
    const res = await app.request("/u", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify([{ id: 1, label: "a" }]),
    });
    expect(res.status).toBe(200);
    expect(await res.json()).toStrictEqual({ firstId: 1, firstLabel: "a" });
  });

  it("types union() of primitives in c.req.valid('json')", async () => {
    const Schema = union(string(), number());
    const app = new Hono().post("/u", sValidator("json", Schema), (c) => {
      const value: string | number = c.req.valid("json");
      return c.json({ value });
    });
    const res = await app.request("/u", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify("hello"),
    });
    expect(res.status).toBe(200);
    expect(await res.json()).toStrictEqual({ value: "hello" });
  });

  it("types intersection() of objects in c.req.valid('json')", async () => {
    const Schema = intersection(
      object({ id: string() }),
      object({ label: string() }),
    );
    const app = new Hono().post("/u", sValidator("json", Schema), (c) => {
      const input = c.req.valid("json");
      const id: string = input.id;
      const label: string = input.label;
      return c.json({ id, label });
    });
    const res = await app.request("/u", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ id: "i1", label: "l1" }),
    });
    expect(res.status).toBe(200);
    expect(await res.json()).toStrictEqual({ id: "i1", label: "l1" });
  });
});
