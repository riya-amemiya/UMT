import { addition } from './addition';

export const toKelvin = (celsius: number): number =>
    addition(celsius, 273.15);
