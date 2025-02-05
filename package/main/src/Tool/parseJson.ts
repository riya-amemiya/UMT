export const parseJson = <T = unknown>(json: string): T => {
  return JSON.parse(json);
};
