import fs from 'fs';
/**
 * Check if file exists
 *
 * @param {string} file
 * @returns {boolean}
 */
export const check = (file: string): boolean => {
    let hasfaile = false;
    try {
        fs.statSync(file);
        hasfaile = true;
    } catch (err) {
        hasfaile = false;
    }
    return hasfaile;
};
