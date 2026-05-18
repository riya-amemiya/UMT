import { sValidator } from "@hono/standard-validator";
import { Hono } from "hono";
import { arrayOf } from "@/Validate/array/arrayOf";
import { boolean } from "@/Validate/boolean";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { omit_ } from "@/Validate/object/omit";
import { optional } from "@/Validate/object/optional";
import { partial } from "@/Validate/object/partial";
import { pick_ } from "@/Validate/object/pick";
import { required } from "@/Validate/object/required";
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

  it("types c.req.valid('json') from an omit_() derived schema", async () => {
    const Base = object({ id: string(), name: string(), age: number() });
    const Schema = omit_(Base, ["age"]);
    const app = new Hono().post("/u", sValidator("json", Schema), (c) => {
      const input = c.req.valid("json");
      const id: string = input.id;
      const name: string = input.name;
      return c.json({ id, name });
    });
    const res = await app.request("/u", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ id: "x", name: "n" }),
    });
    expect(res.status).toBe(200);
    expect(await res.json()).toStrictEqual({ id: "x", name: "n" });
  });

  it("types c.req.valid('json') from a pick_() derived schema", async () => {
    const Base = object({ id: string(), name: string(), age: number() });
    const Schema = pick_(Base, ["id"]);
    const app = new Hono().post("/u", sValidator("json", Schema), (c) => {
      const input = c.req.valid("json");
      const id: string = input.id;
      return c.json({ id });
    });
    const res = await app.request("/u", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ id: "x" }),
    });
    expect(res.status).toBe(200);
    expect(await res.json()).toStrictEqual({ id: "x" });
  });

  it("types c.req.valid('json') from a partial() derived schema", async () => {
    const Base = object({ id: string(), name: string() });
    const Schema = partial(Base);
    const app = new Hono().post("/u", sValidator("json", Schema), (c) => {
      const input = c.req.valid("json");
      const id: string | undefined = input.id;
      const name: string | undefined = input.name;
      return c.json({ id: id ?? null, name: name ?? null });
    });
    const res = await app.request("/u", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({}),
    });
    expect(res.status).toBe(200);
    expect(await res.json()).toStrictEqual({ id: null, name: null });
  });

  it("types c.req.valid('json') from a required() derived schema", async () => {
    const Base = object({ id: string(), nick: optional(string()) });
    const Schema = required(Base);
    const app = new Hono().post("/u", sValidator("json", Schema), (c) => {
      const input = c.req.valid("json");
      const id: string = input.id;
      const nick: string = input.nick;
      return c.json({ id, nick });
    });
    const res = await app.request("/u", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ id: "x", nick: "n" }),
    });
    expect(res.status).toBe(200);
    expect(await res.json()).toStrictEqual({ id: "x", nick: "n" });
  });

  it("types c.req.valid('json') from a partial(omit_()) chained derivation", async () => {
    const Base = object({ id: string(), name: string(), age: number() });
    const Schema = partial(omit_(Base, ["age"]));
    const app = new Hono().post("/u", sValidator("json", Schema), (c) => {
      const input = c.req.valid("json");
      const id: string | undefined = input.id;
      const name: string | undefined = input.name;
      return c.json({ id: id ?? null, name: name ?? null });
    });
    const res = await app.request("/u", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ id: "x" }),
    });
    expect(res.status).toBe(200);
    expect(await res.json()).toStrictEqual({ id: "x", name: null });
  });
});
