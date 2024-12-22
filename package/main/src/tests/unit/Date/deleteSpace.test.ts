import { deleteSpaces } from "@/String/deleteSpaces";

describe("deleteSpace", () => {
  it("should remove all spaces from a string", () => {
    const input = "hello world";
    const expectedOutput = "helloworld";
    const actualOutput = deleteSpaces(input);
    expect(actualOutput).toEqual(expectedOutput);
  });

  it("should remove all spaces from an empty string", () => {
    const input = "";
    const expectedOutput = "";
    const actualOutput = deleteSpaces(input);
    expect(actualOutput).toEqual(expectedOutput);
  });

  it("should remove all spaces from a string with leading and trailing spaces", () => {
    const input = "  hello world  ";
    const expectedOutput = "helloworld";
    const actualOutput = deleteSpaces(input);
    expect(actualOutput).toEqual(expectedOutput);
  });

  it("should remove all spaces from a string with multiple spaces between words", () => {
    const input = "hello   world";
    const expectedOutput = "helloworld";
    const actualOutput = deleteSpaces(input);
    expect(actualOutput).toEqual(expectedOutput);
  });

  it("should remove all spaces from a string with tabs and newlines", () => {
    const input = "hello\t\nworld";
    const expectedOutput = "helloworld";
    const actualOutput = deleteSpaces(input);
    expect(actualOutput).toEqual(expectedOutput);
  });
});
