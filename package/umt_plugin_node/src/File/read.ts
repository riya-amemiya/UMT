import fs from 'fs';
import { check } from './check';
/**
 * Read file
 * @param file
 * @returns {string}
 */
export const read = (file: string): string => {
    if (check(file)) {
        return fs.readFileSync(file, 'utf8');
    }
    return '';
};
