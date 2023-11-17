import { trimEndCharacters } from "./trimEndCharacters";
import { trimStartCharacters } from "./trimStartCharacters";

/**
 * 指定された文字列から、指定された文字を前後から削除します。
 *
 * @param {string} string_ - トリム対象の文字列。
 * @param {string} chars - 削除する文字のセット。
 * @returns {string} 前後から指定された文字が削除された新しい文字列。
 */
export const trimCharacters = (string_: string, chars: string): string => {
  return trimEndCharacters(trimStartCharacters(string_, chars), chars);
};
