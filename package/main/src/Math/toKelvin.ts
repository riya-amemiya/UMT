import { addition } from './addition';

const toKelvin = (celsius: number): number =>
    addition(celsius, 273.15);
export { toKelvin };
