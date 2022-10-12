import { subtract } from './subtract';

export const toCelsius = (kelvin: number) => subtract(kelvin, 273.15);
