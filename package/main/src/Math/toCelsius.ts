import { subtract } from './subtract';

const toCelsius = (kelvin: number) => subtract(kelvin, 273.15);
export { toCelsius };
