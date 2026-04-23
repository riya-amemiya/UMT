import { deepClone } from "@/Object/deepClone";
import { getObjectsCommon } from "@/Object/getObjectsCommon";
import { getObjectsDiff } from "@/Object/getObjectsDiff";
import { mapKeys } from "@/Object/mapKeys";
import { mapValues } from "@/Object/mapValues";
import { merge } from "@/Object/merge";
import { mergeDeep } from "@/Object/mergeDeep";
import { pick } from "@/Object/pick";
import { pickDeep } from "@/Object/pickDeep";
import { removePrototype } from "@/Object/removePrototype";
import { removePrototypeDeep } from "@/Object/removePrototypeDeep";
import { removePrototypeMap } from "@/Object/removePrototypeMap";
import { removePrototypeMapDeep } from "@/Object/removePrototypeMapDeep";

type PollutionProbe = {
  leaked?: unknown;
  injected?: unknown;
  polluted?: unknown;
};

/**
 * Integration tests for the prototype-pollution defense chain.
 *
 * These tests verify two things together:
 *   1. That naive user code (equivalent to popular vulnerable patterns)
 *      actually pollutes the global Object.prototype when fed JSON-parsed
 *      attacker payloads. This proves the attack surface is real.
 *   2. That combining the library's sanitizers with the Object helpers
 *      neutralizes the attack end-to-end.
 */
describe("Integration test for prototype pollution defense", () => {
  // All tests that pollute Object.prototype must clean up in afterEach —
  // otherwise the contamination leaks to other test files.
  afterEach(() => {
    Reflect.deleteProperty(Object.prototype, "leaked");
    Reflect.deleteProperty(Object.prototype, "injected");
    Reflect.deleteProperty(Object.prototype, "polluted");
  });

  const naiveDeepMerge = (
    target: Record<string, unknown>,
    source: Record<string, unknown>,
  ): Record<string, unknown> => {
    for (const key of Object.keys(source)) {
      const sourceValue = source[key];
      const targetValue = target[key];
      const targetIsObjectLike =
        targetValue !== null &&
        (typeof targetValue === "object" || typeof targetValue === "function");
      if (
        sourceValue !== null &&
        typeof sourceValue === "object" &&
        !Array.isArray(sourceValue) &&
        targetIsObjectLike
      ) {
        naiveDeepMerge(
          targetValue as Record<string, unknown>,
          sourceValue as Record<string, unknown>,
        );
      } else {
        target[key] = sourceValue;
      }
    }
    return target;
  };

  it("should globally pollute via __proto__ with a naive deep merge, and prevent it with removePrototypeDeep + mergeDeep", () => {
    const payload = JSON.parse('{"__proto__":{"leaked":"yes"},"ok":1}');

    const victim: Record<string, unknown> = {};
    naiveDeepMerge(victim, payload);

    const innocent: PollutionProbe = {};
    expect(innocent.leaked).toBe("yes");

    Reflect.deleteProperty(Object.prototype, "leaked");

    const safe = mergeDeep({}, removePrototypeDeep(payload)) as PollutionProbe;
    const afterSafe: PollutionProbe = {};
    expect(afterSafe.leaked).toBeUndefined();
    expect(safe.leaked).toBeUndefined();
    expect(safe).toEqual({ ok: 1 });
  });

  it("should globally pollute via constructor.prototype, and prevent it via removePrototypeDeep", () => {
    const payload = JSON.parse(
      '{"constructor":{"prototype":{"injected":"deep"}},"ok":1}',
    );

    const victim: Record<string, unknown> = {};
    naiveDeepMerge(victim, payload);

    const innocent: PollutionProbe = {};
    expect(innocent.injected).toBe("deep");

    Reflect.deleteProperty(Object.prototype, "injected");

    const sanitized = removePrototypeDeep(payload);
    const afterVictim: Record<string, unknown> = {};
    naiveDeepMerge(afterVictim, sanitized);
    const afterSafe: PollutionProbe = {};
    expect(afterSafe.injected).toBeUndefined();
    expect(sanitized).toEqual({ ok: 1 });
  });

  it("should sanitize an entire request batch before merging configs", () => {
    const batch = [
      JSON.parse(
        '{"__proto__":{"leaked":"req1"},"app":{"name":"MyApp","flags":{"a":true}}}',
      ),
      JSON.parse('{"app":{"flags":{"b":true},"__proto__":{"leaked":"req2"}}}'),
      JSON.parse(
        '{"constructor":{"prototype":{"injected":"req3"}},"app":{"version":"2.0"}}',
      ),
    ];

    const sanitized = removePrototypeMapDeep(batch);
    const merged = mergeDeep({}, ...sanitized);

    expect(merged).toEqual({
      app: {
        name: "MyApp",
        flags: { a: true, b: true },
        version: "2.0",
      },
    });

    const innocent: PollutionProbe = {};
    expect(innocent.leaked).toBeUndefined();
    expect(innocent.injected).toBeUndefined();
  });

  it("should keep deepClone output free of polluted prototypes when the payload is sanitized first", () => {
    const payload = JSON.parse(
      '{"a":{"__proto__":{"leaked":"x"},"value":1},"b":[{"__proto__":{"leaked":"y"},"item":2}]}',
    );

    const cloned = deepClone(removePrototypeDeep(payload));

    expect(cloned).toEqual({ a: { value: 1 }, b: [{ item: 2 }] });

    const innocent: PollutionProbe = {};
    expect(innocent.leaked).toBeUndefined();
  });

  it("should chain pick -> mergeDeep -> deepClone safely on user-controlled input", () => {
    const userInput = JSON.parse(
      '{"__proto__":{"leaked":"pick"},"name":"Alice","age":30,"secret":"top"}',
    );

    const visible = pick(removePrototype(userInput), "name", "age");
    const profile = mergeDeep({ role: "user" }, visible);
    const snapshot = deepClone(profile);

    expect(snapshot).toEqual({ role: "user", name: "Alice", age: 30 });

    const innocent: PollutionProbe = {};
    expect(innocent.leaked).toBeUndefined();
  });

  it("should safely extract nested fields with pickDeep when input is sanitized", () => {
    const payload = JSON.parse(
      '{"user":{"__proto__":{"leaked":"nested"},"profile":{"name":"Bob","email":"bob@example.com"}}}',
    );

    const sanitized = removePrototypeDeep(payload) as {
      user: { profile: { name: string; email: string } };
    };
    const result = pickDeep(sanitized, "user.profile.name");

    expect(result).toEqual({ user: { profile: { name: "Bob" } } });

    const innocent: PollutionProbe = {};
    expect(innocent.leaked).toBeUndefined();
  });

  it("should not leak across helper boundaries: shallow sanitization is not enough for deep payloads", () => {
    const payload = JSON.parse(
      '{"outer":{"__proto__":{"leaked":"deep"},"value":1}}',
    );

    // Shallow `removePrototype` only clears the top-level dangerous keys;
    // the nested __proto__ survives. Demonstrates why removePrototypeDeep
    // is required when the payload is deeper than one level.
    const shallow = removePrototype(payload) as {
      outer: Record<string, unknown>;
    };
    expect(Object.hasOwn(shallow.outer, "__proto__")).toBe(true);

    // Deep sanitization removes all of them.
    const deep = removePrototypeDeep(payload) as {
      outer: Record<string, unknown>;
    };
    expect(Object.hasOwn(deep.outer, "__proto__")).toBe(false);
  });

  it("should combine removePrototypeMap with merge for arrays of shallow user inputs", () => {
    const entries = [
      JSON.parse('{"__proto__":{"leaked":"a"},"k":"value1"}'),
      JSON.parse('{"constructor":{"polluted":true},"k":"value2"}'),
      JSON.parse('{"prototype":{"hack":true},"k":"value3"}'),
    ];

    const sanitized = removePrototypeMap(entries);
    const combined = merge({}, ...sanitized);

    expect(combined).toEqual({ k: "value3" });

    const innocent: PollutionProbe = {};
    expect(innocent.leaked).toBeUndefined();
  });

  it("should keep diff and common helpers safe when combined with removePrototypeDeep", () => {
    const a = JSON.parse(
      '{"__proto__":{"leaked":"a"},"kept":1,"changed":"x","nested":{"only":"a"}}',
    );
    const b = JSON.parse(
      '{"__proto__":{"leaked":"b"},"kept":1,"changed":"y","nested":{"shared":2}}',
    );

    const safeA = removePrototypeDeep(a);
    const safeB = removePrototypeDeep(b);

    const common = getObjectsCommon(safeA, safeB);
    const diff = getObjectsDiff(safeA, safeB);

    expect(common).toEqual({ kept: 1 });
    expect(diff).toEqual({
      changed: "y",
      nested: { only: "a", shared: 2 },
    });

    const innocent: PollutionProbe = {};
    expect(innocent.leaked).toBeUndefined();
  });

  it("should ensure mapKeys/mapValues output never exposes __proto__ as an own key with sanitized input", () => {
    const payload = JSON.parse('{"__proto__":{"leaked":"x"},"a":1,"b":2}');

    const sanitized = removePrototype(payload);
    const renamed = mapKeys(sanitized, (_value, key) => key.toUpperCase());
    const doubled = mapValues(renamed, (value) => (value as number) * 2);

    expect(doubled).toEqual({ A: 2, B: 4 });
    expect(Object.hasOwn(doubled, "__proto__")).toBe(false);

    const innocent: PollutionProbe = {};
    expect(innocent.leaked).toBeUndefined();
  });

  it("should catch a real-world lodash-style deep merge scenario end-to-end", () => {
    // Simulates a config endpoint that merges user overrides into defaults.
    const defaults = {
      service: { port: 3000, features: { cache: true } },
    };
    const userOverrides = JSON.parse(
      '{"service":{"port":8080,"__proto__":{"leaked":"port"}},"__proto__":{"leaked":"root"}}',
    );

    // Without sanitization, a naive deep merge would pollute Object.prototype.
    const victim: Record<string, unknown> = {};
    naiveDeepMerge(victim, userOverrides);
    const innocentDuringAttack: PollutionProbe = {};
    expect(innocentDuringAttack.leaked).toBeDefined();

    Reflect.deleteProperty(Object.prototype, "leaked");

    // With sanitization, no leak.
    const safeConfig = mergeDeep(defaults, removePrototypeDeep(userOverrides));
    expect(safeConfig).toEqual({
      service: { port: 8080, features: { cache: true } },
    });

    const innocentAfterSafe: PollutionProbe = {};
    expect(innocentAfterSafe.leaked).toBeUndefined();
  });
});
