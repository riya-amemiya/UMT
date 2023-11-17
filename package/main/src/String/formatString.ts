/**
 * テンプレート文字列内のプレースホルダーを指定された値で置換します。
 * プレースホルダーは {0}、{1}、{2}... の形式で、対応する値が配列で渡されます。
 *
 * @param template - プレースホルダーを含むテンプレート文字列
 * @param values - テンプレート内のプレースホルダーに置換する値の配列
 * @returns プレースホルダーが値で置換された文字列
 *
 * @example
 * // "こんにちは、世界！" を返します
 * formatString("こんにちは、{0}！", "世界");
 */
export const formatString = <T>(template: string, ...values: T[]) => {
  return template.replaceAll(/{(\d+)}/g, (match, index) => {
    return values[index] === undefined ? match : String(values[index]);
  });
};
