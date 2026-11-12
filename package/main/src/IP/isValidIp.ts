const IPV4_PATTERN =
  /^(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}$/;

/**
Checks whether a string is a valid IPv4 address in dotted-quad notation
@param {string} ip - String to check (e.g., "192.168.1.1")
@returns {boolean} True if the string has exactly four decimal octets in the range 0-255 without leading zeros, false otherwise
@example isValidIp("192.168.1.1"); // true
@example isValidIp("192.168.01.1"); // false
@example isValidIp("256.0.0.1"); // false
*/
export const isValidIp = (ip: string): boolean => IPV4_PATTERN.test(ip);
