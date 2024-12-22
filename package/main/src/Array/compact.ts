export const compact = <T>(array: T[]): T[] => array.filter(Boolean) as T[];
