export type ParseEmailLevel =
  | "basic"
  | "rfc822"
  | "rfc2822"
  | "rfc5321"
  | "rfc5322";

const EMAIL_PATTERNS = {
  basic:
    /^(?<local>[\w!#$%&'*+./=?^`{|}~-]+)@(?<domain>[\dA-Za-z](?:[\dA-Za-z-]{0,61}[\dA-Za-z])?(?:\.[\dA-Za-z](?:[\dA-Za-z-]{0,61}[\dA-Za-z])?)*)$/,
  rfc822:
    /^(?:\s|\((?:[^()\\]|\\[\S\s])*(?:\((?:[^()\\]|\\[\S\s])*\)(?:[^()\\]|\\[\S\s])*)*\))*(?<local>"(?:[^"\\]|\\[\S\s]){0,62}"|[\w!#$%&'*/=?^`{|}~-]{1,64}(?:\.[\w!#$%&'*/=?^`{|}~-]+)*)(?:\s|\((?:[^()\\]|\\[\S\s])*(?:\((?:[^()\\]|\\[\S\s])*\)(?:[^()\\]|\\[\S\s])*)*\))*@(?:\s|\((?:[^()\\]|\\[\S\s])*(?:\((?:[^()\\]|\\[\S\s])*\)(?:[^()\\]|\\[\S\s])*)*\))*(?<domain>[\dA-Za-z](?:[\dA-Za-z-]{0,61}[\dA-Za-z])?(?:\.[\dA-Za-z](?:[\dA-Za-z-]{0,61}[\dA-Za-z])?)*)(?:\s|\((?:[^()\\]|\\[\S\s])*(?:\((?:[^()\\]|\\[\S\s])*\)(?:[^()\\]|\\[\S\s])*)*\))*$/,
  rfc2822:
    /^(?=.{1,998}$)(?!.*\.\.)(?<local>[\w!#$%&'*/=?^`{|}~-](?:[\w!#$%&'*+./=?^`{|}~-]{0,62}[\w!#$%&'*/=?^`{|}~-])?)@(?<domain>[\dA-Za-z](?:[\dA-Za-z-]{0,61}[\dA-Za-z])?(?:\.[\dA-Za-z](?:[\dA-Za-z-]{0,61}[\dA-Za-z])?)*\.[A-Za-z]{2,})$/,
  rfc5321:
    /^(?=.{1,256}$)(?=[^@]{1,64}@)(?!.*\.\.)(?<local>(?:[\w!#$%&'*+/=?^`{|}~-]+(?:\.[\w!#$%&'*+/=?^`{|}~-]+)*|"(?:[^"\\]|\\[\S\s]){0,62}"))@(?<domain>[\dA-Za-z](?:[\dA-Za-z-]{0,61}[\dA-Za-z])?(?:\.[\dA-Za-z](?:[\dA-Za-z-]{0,61}[\dA-Za-z])?)+|\[(?:(?:\d{1,3}\.){3}\d{1,3}|IPv6:[\d:A-Fa-f]+)])$/,
  rfc5322:
    /^(?=.{1,998}$)(?:\s|\((?:[^()\\]|\\[\S\s])*(?:\((?:[^()\\]|\\[\S\s])*\)(?:[^()\\]|\\[\S\s])*)*\))*(?<local>"(?:[^"\\]|\\[\S\s]){0,62}"(?:\."(?:[^"\\]|\\[\S\s]){0,62}")*|"(?:[^"\\]|\\[\S\s]){0,62}"(?:(?:\.[\w!#$%&'*+/=?^`{|}~-]{1,64})+)+|[\w!#$%&'*+/=?^`{|}~-]{1,64}(?:\.[\w!#$%&'*+/=?^`{|}~-]{1,64})*(?:\."(?:[^"\\]|\\[\S\s]){0,62}")+|[\w!#$%&'*+/=?^`{|}~-]{1,64}(?:(?:\s|\((?:[^()\\]|\\[\S\s])*(?:\((?:[^()\\]|\\[\S\s])*\)(?:[^()\\]|\\[\S\s])*)*\))*\.(?:\s|\((?:[^()\\]|\\[\S\s])*(?:\((?:[^()\\]|\\[\S\s])*\)(?:[^()\\]|\\[\S\s])*)*\))*[\w!#$%&'*+/=?^`{|}~-]{1,64})*)(?:\s|\((?:[^()\\]|\\[\S\s])*(?:\((?:[^()\\]|\\[\S\s])*\)(?:[^()\\]|\\[\S\s])*)*\))*@(?:\s|\((?:[^()\\]|\\[\S\s])*(?:\((?:[^()\\]|\\[\S\s])*\)(?:[^()\\]|\\[\S\s])*)*\))*(?<domain>[\dA-Za-z](?:[\dA-Za-z-]{0,61}[\dA-Za-z])?(?:(?:\s|\((?:[^()\\]|\\[\S\s])*(?:\((?:[^()\\]|\\[\S\s])*\)(?:[^()\\]|\\[\S\s])*)*\))*\.(?:\s|\((?:[^()\\]|\\[\S\s])*(?:\((?:[^()\\]|\\[\S\s])*\)(?:[^()\\]|\\[\S\s])*)*\))*[\dA-Za-z](?:[\dA-Za-z-]{0,61}[\dA-Za-z])?)+|\[(?:(?:\d{1,3}\.){3}\d{1,3}|IPv6:[\d:A-Fa-f]+)])(?:\s|\((?:[^()\\]|\\[\S\s])*(?:\((?:[^()\\]|\\[\S\s])*\)(?:[^()\\]|\\[\S\s])*)*\))*$/,
} as const;

export interface ParseEmailOptions {
  level: ParseEmailLevel;
}

export const parseEmail = ({
  email,
  options,
}: {
  email: string;
  options: ParseEmailOptions;
}): {
  valid: boolean;
  parts?: { local: string; domain: string };
} => {
  // ReDoS mitigation: reject excessively long inputs before regex evaluation
  // RFC 5321 specifies max 256 characters for a full email address
  const MAX_EMAIL_LENGTH = 320;
  if (email.length > MAX_EMAIL_LENGTH) {
    return { valid: false };
  }

  const { level } = options;
  const pattern = EMAIL_PATTERNS[level];
  const match = pattern.exec(email);

  return {
    valid: match !== null,
    parts: match?.groups
      ? {
          // biome-ignore lint/complexity/useLiteralKeys: Literal keys are used for group names
          local: match.groups["local"],
          // biome-ignore lint/complexity/useLiteralKeys: Literal keys are used for group names
          domain: match.groups["domain"],
        }
      : undefined,
  };
};
