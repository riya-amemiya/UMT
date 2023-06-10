import { repeatedTrial } from "../../module/Math/repeatedTrial";
test("{repeatedTrial}", () => {
  expect(repeatedTrial(4, 2, { x: 1, y: 3 })).toEqual(
    expect.arrayContaining([8, 27]),
  );
});
