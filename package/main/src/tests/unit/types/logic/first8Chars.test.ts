import type { First8Chars } from "$/logic/first8Chars";

describe("First8Chars", () => {
  it('First8Chars<"abcdefghij">', () => {
    const _: First8Chars<"abcdefghij"> = "abcdefgh";
    expect(_).toEqual(_);
  });
  it('First8Chars<"abcdefgh">', () => {
    const _: First8Chars<"abcdefgh"> = "abcdefgh";
    expect(_).toEqual(_);
  });
  it('First8Chars<"abc">', () => {
    const _: [First8Chars<"abc">] extends [never] ? true : false = true;
    expect(_).toEqual(_);
  });
});
