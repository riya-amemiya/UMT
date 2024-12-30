import { repeatedTrial } from "@/Math/repeatedTrial";
test("{repeatedTrial}", () => {
  expect(repeatedTrial(4, 2, { x: 1, y: 3 })).toEqual([8, 27]);
});
