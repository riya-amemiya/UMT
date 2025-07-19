# UMT Main Package

UMT Main Package is written in TypeScript and is a collection of useful functions for various tasks.

## Install

```bash
npm install umt

# or

yarn add umt

# or

pnpm add umt

# or

bun add umt
```

## Function List

### Advance

| name | type | description | example |
|------|------|-------------|---------|
| rangeAdvance | `(start: number, end?: number, conditionalExpression?: (n: number) => boolean) => number[]` | Returns an array of numbers that satisfy the conditional expression | `rangeAdvance(1, 10, (n) => n % 2 === 0); // [2, 4, 6, 8]` |

### Array

| name | type | description | example |
|------|------|-------------|---------|
| arraysJoin | `<A extends unknown[]>(array: unknown[], ...arrays: unknown[]) => A` | Join arrays without duplicates | `arraysJoin([1, 2, 3], [2, 3, 4]); // [1, 2, 3, 4]` |
| binarySearch | `(array: number[], target: number) => number` | Binary search implementation | `binarySearch([1, 2, 3, 4, 5], 3); // 2` |
| checkFlagAlignment | `<T extends { flag: boolean }>(matrix: T[][]) => boolean` | Check if flags are aligned in any direction (horizontal, vertical, or diagonal) | `checkFlagAlignment([[{flag: true}, {flag: true}]]); // true` |
| chunk | `<T extends unknown[], N extends number>(array: T, n: N) => ChunkArrayType<T, N>` | Split an array into smaller chunks of specified size | `chunk([1, 2, 3, 4, 5, 6, 7, 8, 9], 3); // [[1, 2, 3], [4, 5, 6], [7, 8, 9]]` |
| compact | `<T>(array: T[]) => T[]` | Creates an array with all falsey values removed | `compact([0, 1, false, 2, '', 3]); // [1, 2, 3]` |
| drop | `<T>(array: T[], n?: number, direction?: "left" \| "right" \| "between") => T[]` | Returns a new array with n elements removed from the specified direction | `drop([1, 2, 3, 4, 5], 2); // [3, 4, 5]` |
| dualPivotQuickSort | `<T>(array: T[], compareFunction?: CompareFunction<T>, startIndex?: number, endIndex?: number, insertionSortThreshold?: number) => T[]` | Sort array using dual-pivot quicksort algorithm | `dualPivotQuickSort([3, 1, 4, 1, 5, 9, 2, 6, 5, 3]); // [1, 1, 2, 3, 3, 4, 5, 5, 6, 9]` |
| first | `<T extends unknown[]>(array: T) => First<T>` | Returns the first element of an array | `first([1, 2, 3]); // 1` |
| generateNumberArray | `(length: number, min?: number, max?: number, random?: boolean) => number[]` | Generates an array of numbers with the specified length | `generateNumberArray(5); // [0, 1, 2, 3, 4]` |
| getArraysCommon | `<O, T extends unknown[]>(array: T, ...arrays: T[]) => O` | Extract common elements from multiple arrays | `getArraysCommon([1, 2, 3], [2, 3, 4], [2, 5, 3]); // [2, 3]` |
| getArraysDiff | `<O, T extends unknown[]>(array: T, ...arrays: T[]) => O` | Extract elements that are not common between arrays | `getArraysDiff([1, 2, 3], [2, 3, 4]); // [1, 4]` |
| groupBy | `<T, K extends string \| number>(array: T[], iteratee: (value: T, index: number, array: T[]) => K) => Record<K, T[]>` | Groups elements of an array based on a given iteratee function | `groupBy([6.1, 4.2, 6.3], Math.floor); // { '4': [4.2], '6': [6.1, 6.3] }` |
| insertionSort | `<T>(array: T[], compareFunction?: CompareFunction<T>, start?: number, end?: number) => T[]` | Sort an array using insertion sort algorithm | `insertionSort([4, 2, 7, 1, 3]); // [1, 2, 3, 4, 7]` |
| mergeSort | `<T>(array: T[], compareFunction?: CompareFunction<T>) => T[]` | Merge sort implementation | `mergeSort([1, 3, 2, 4, 5], (a, b) => a - b); // [1, 2, 3, 4, 5]` |
| pop | `<T>(array: T[]) => T \| undefined` | Removes the last element from an array and returns it | `pop([1, 2, 3]); // 3` |
| quickSort | `<T>(array: T[], compareFunction?: CompareFunction<T>, startIndex?: number, endIndex?: number, insertionSortThreshold?: number) => T[]` | Sorts an array using a hybrid algorithm combining QuickSort and InsertionSort | `quickSort([1, 3, 2, 4, 5]); // [1, 2, 3, 4, 5]` |
| randomSelect | `<T>(array: T[], count: number, allowDuplicates?: boolean) => T[]` | Randomly selects a specified number of elements from an array | `randomSelect([1, 2, 3, 4, 5], 2); // [3, 1]` |
| range | `(start: number, end?: number, step?: number) => number[]` | Generates an array of sequential numbers | `range(5); // [0, 1, 2, 3, 4]` |
| shuffle | `<T>(array: T[]) => T[]` | Randomly shuffles the elements of an array | `shuffle([1, 2, 3, 4, 5]); // [3, 5, 2, 4, 1]` |
| shuffle2DArray | `<T>(array: T[][]) => T[][]` | Shuffles all elements in a 2D array while maintaining the row lengths | `shuffle2DArray([[1, 2], [3, 4], [5, 6]]); // [[1, 3], [6, 4], [2, 5]]` |
| sum | `(x: number[]) => number` | Returns the sum of an array of numbers | `sum([1, 2, 3]); // 6` |
| timSort | `<T>(array: T[], compareFunction?: CompareFunction<T>, start?: number, end?: number) => T[]` | Implementation of the TimSort algorithm | `timSort([3, 1, 4, 1, 5]); // [1, 1, 3, 4, 5]` |
| ultraNumberSort | `(array: number[], ascending?: boolean) => number[]` | Ultra-fast sorting specifically optimized for number arrays | `ultraNumberSort([3, 1, 4, 1, 5, 9, 2, 6, 5, 3]); // [1, 1, 2, 3, 3, 4, 5, 5, 6, 9]` |
| uniqBy | `<T, K>(array: T[], selector: (item: T) => K) => T[]` | Removes duplicate values from an array based on a selector function | `uniqBy([{id: 1}, {id: 2}, {id: 1}], item => item.id); // [{id: 1}, {id: 2}]` |
| unique | `<T>(array: T[]) => T[]` | Removes duplicate values from an array | `unique([1, 2, 2, 3, 3]); // [1, 2, 3]` |
| zip | `<T extends unknown[][]>(...arrays: T) => ZipArrayType<T>` | Creates a new array by combining elements from multiple arrays at corresponding positions | `zip([1, 2], ['a', 'b']); // [[1, 'a'], [2, 'b']]` |
| zipLongest | `<T extends unknown[][]>(...arrays: T) => ZipArrayType<T>` | Combines arrays of different lengths by padding shorter arrays with undefined values | `zipLongest([1, 2], ['a']); // [[1, 'a'], [2, undefined]]` |

### DataStructure

| name | type | description | example |
|------|------|-------------|---------|
| PriorityQueue | `class PriorityQueue<T>` | A priority queue implementation using a binary heap. Higher priority values are dequeued first. | `const queue = new PriorityQueue<string>(); queue.enqueue("low", 1); queue.enqueue("high", 3); queue.enqueueBack("back"); queue.dequeue(); // "high"` |

### Color

| name | type | description | example |
|------|------|-------------|---------|
| cmykToRgba | `(c: number, m: number, y: number, k: number, a?: number) => { r: number; g: number; b: number; a: number }` | Convert CMYK color values to RGBA color space | `cmykToRgba(100, 100, 0, 60.78) // { r: 0, g: 0, b: 100, a: 1 }` |
| hexaToRgba | `(hex: string) => { r: number; g: number; b: number; a: number }` | Convert hexadecimal color code to RGBA color values | `hexToRgba("#00000000") // { r: 0, g: 0, b: 0, a: 0 }` |
| hslaToRgba | `(h: number, s: number, l: number, a?: number) => { r: number; g: number; b: number; a: number }` | Convert HSLA color values to RGBA color space | `hslaToRgba(120, 50, 50, 1) // { r: 64, g: 191, b: 64, a: 1 }` |
| rgbaToCmyk | `(rgba: { r: number; g: number; b: number; a?: number }) => { c: number; m: number; y: number; k: number; a: number }` | Convert RGBA color to CMYK color model | `rgbaToCmyk({ r: 0, g: 0, b: 0, a: 1 }); // { c: 0, m: 0, y: 0, k: 100, a: 1 }` |
| rgbaToHexA | `(rgba: { r: number; g: number; b: number; a?: number }) => string` | Convert RGBA color to hexadecimal color code | `rgbaToHexA({ r: 0, g: 0, b: 0, a: 1 }); // "#000000ff"` |
| rgbaToHsla | `(rgba: { r: number; g: number; b: number; a?: number }) => { h: number; s: number; l: number; a: number }` | Convert RGBA color values to HSLA color space | `rgbaToHsla({ r: 100, g: 100, b: 100, a: 1 }); // { h: 0, s: 0, l: 39.22, a: 1 }` |

### Crypto

| name | type | description | example |
|------|------|-------------|---------|
| decodeBase32 | `(input: string) => Uint8Array` | Decodes an uppercase Base32 string to Uint8Array. Does not validate padding placement. | `decodeBase32("JBSWY3DP"); // Uint8Array for "Hello"` |
| decodeBase32ToString | `(input: string) => string` | Decodes an uppercase Base32 string to a UTF-8 string. Does not validate padding placement. | `decodeBase32ToString("JBSWY3DP"); // "Hello"` |
| decodeBase58 | `(input: string) => Uint8Array` | Decodes a Base58 string to Uint8Array | `decodeBase58("9Ajdvzr"); // Uint8Array for "Hello"` |
| decodeBase58ToString | `(input: string) => string` | Decodes a Base58 string to a UTF-8 string | `decodeBase58ToString("9Ajdvzr"); // "Hello"` |
| encodeBase32 | `(input: string \| Uint8Array) => string` | Encodes a string or Uint8Array to Base32 format | `encodeBase32("Hello"); // "JBSWY3DP"` |
| encodeBase58 | `(input: string \| Uint8Array) => string` | Encodes a string or Uint8Array to Base58 format | `encodeBase58("Hello"); // "9Ajdvzr"` |

### Date

| name | type | description | example |
|------|------|-------------|---------|
| birthday | `<T extends MonTypeInt>(year: number, mon: T, day: DayTypeInt<T>, timeDifference?: HoursTypeInt) => number` | Calculate age based on birthdate | `birthday(2000, 1, 1); // Returns age of someone born on Jan 1, 2000` |
| dateRange | `(startDate: Date, endDate: Date) => Date[]` | Generate an array containing all dates between the specified start and end dates | `dateRange(new Date('2025-01-01'), new Date('2025-01-03'))` |
| dayOfWeek | `<T extends MonTypeInt>(properties?: { year?: number; mon?: T; day?: DayTypeInt<T> }, timeDifference?: HoursTypeInt) => number` | Get the day of the week | `dayOfWeek({ year: 2000, mon: 1, day: 1 });` |
| format | `(date: Date, formatString?: string) => string` | Converts a date to a string according to the specified format pattern | `format(new Date('2025-04-04'), 'YYYY-MM-DD') // Returns "2025-04-04"` |
| getDay | `<T extends keyof DayList>(day: number, lang?: T) => ArrayToUnion<DayList[T]>` | Convert a number to a day of the week in the specified language | `getDay(0, "en"); // Returns "Sun"` |
| getTimezoneOffsetString | `(instance: Date) => string` | Get timezone offset string in format "+HH:mm" or "-HH:mm" | `getTimezoneOffsetString(new Date()); // "+09:00" for JST` |
| isLeapYear | `(year: number) => boolean` | Determine if a given year is a leap year | `isLeapYear(2020); // Returns true` |
| newDateInt | `<T extends MonTypeInt>(year: number, mon: T, day: DayTypeInt<T>, hours?: HoursTypeInt, minutes?: MinutesTypeInt, seconds?: SecondsTypeInt, milliseconds?: MillisecondsTypeInt) => Date` | Create a new Date object from numeric values | `newDateInt(2021, 1, 1); // Creates date for January 1, 2021` |
| newDateString | `<T extends MonTypeZero>(date: string, hours?: HoursType, minutes?: MinutesType, seconds?: SecondsType, milliseconds?: MillisecondsType, timeDifference?: HoursType) => Date` | Create a new Date object from a string date and time components | `newDateString("2021-01-01"); // Creates date for January 1, 2021 00:00:00` |
| now | `(timeDifference?: HoursTypeInt) => Date` | Get the current time with a specified UTC offset | `now(); // Returns current time in JST (UTC+9)` |

### Error

| name | type | description | example |
|------|------|-------------|---------|
| safeExecute | `<V, E = Error>(callback: () => V) => Result<V, E>` | Safely executes a callback function and returns a Result type | `safeExecute(() => JSON.parse('{"a": 1}')); // {type: "success", value: {a: 1}}` |

### Function

| name | type | description | example |
|------|------|-------------|---------|
| curry | `(func: (...args: unknown[]) => unknown) => Function` | Curries a function | `const add = (a, b, c) => a + b + c; curry(add)(1)(2)(3); // 6` |

### IP

| name | type | description | example |
|------|------|-------------|---------|
| cidrToLong | `(cidr: number) => number` | Converts CIDR notation to a subnet mask number | `cidrToLong(24); // 4294967040` |
| cidrToSubnetMask | `(cidr: number) => string` | Converts CIDR notation to a subnet mask | `cidrToSubnetMask(24); // "255.255.255.0"` |
| getIpClass | `(ip: string) => string` | Gets the IP address class (A, B, C, D, or E) | `getIpClass("192.168.1.1"); // "C"` |
| getNetworkAddress | `(ip: string, subnetMask: string) => number` | Calculates the network address from an IP address and subnet mask | `getNetworkAddress("192.168.1.1", "255.255.255.0"); // 3232235776` |
| ipToBinaryString | `(ip: string) => string` | Converts an IPv4 address to its binary string representation | `ipToBinaryString("192.168.1.1"); // "11000000101010000000000100000001"` |
| ipToLong | `(ip: string) => number` | Converts an IPv4 address to a 32-bit number | `ipToLong("192.168.1.1"); // 3232235777` |
| isInRange | `(remoteIp: string, networkIp: string, cidr: number) => boolean` | Checks if an IP address is within a specified network range | `isInRange("192.168.1.100", "192.168.1.0", 24); // true` |
| isPrivateIp | `(ip: string) => boolean` | Checks if an IP address is within private IP ranges | `isPrivateIp("192.168.1.1"); // true` |
| longToIp | `(long: number) => string` | Converts a 32-bit number to an IPv4 address | `longToIp(3232235777); // "192.168.1.1"` |
| subnetMaskToCidr | `(subnetMask: string) => number` | Converts a subnet mask to CIDR notation | `subnetMaskToCidr("255.255.255.0"); // 24` |

### Math

| name | type | description | example |
|------|------|-------------|---------|
| addition | `(...numbers: number[]) => number` | Addition without floating point errors | `addition(0.1, 0.2); // 0.3` |
| average | `(numbers: number[]) => number` | Calculates the arithmetic mean of an array of numbers | `average([1, 2, 3]); // 2` |
| correlationCoefficient | `(x: number[], y: number[]) => number` | Calculate the Pearson correlation coefficient between two arrays | `correlationCoefficient([1, 2, 3, 4, 5], [2, 4, 6, 8, 10]); // 1` |
| bitwise | `(x: number, k: number, direction?: "left" \| "right") => number` | Performs bit rotation on a number | `bitwise(0x12345678, 8); // 0x34567812` |
| calculator | `<T extends Record<string, string \| number>>(expression: string, exchange?: T) => string` | Calculator function that handles mathematical expressions and simple equations | `calculator("1+2"); // "3"` |
| calculatorInitialization | `<T extends { [key: string]: string \| number }>(exchange: T) => (x: string) => string` | Initializes a calculator function with exchange rates | `calculatorInitialization({ $: 100 })("$1"); // "100"` |
| convertCurrency | `<T extends { [key: string]: number \| string }>(inputString: string, conversionRates?: T) => string` | Converts currency amounts in a string using currency symbols | `convertCurrency("¥100", { "¥": 0.01 }); // "1"` |
| degToRad | `(x: number) => number` | Converts degrees to radians | `degToRad(180); // 3.141592653589793` |
| deviationValue | `(value: number, averageValue: number, standardDeviationValue: number) => number` | Calculate standard score (deviation value) | `deviationValue(10, 5, 2); // 75` |
| division | `<T extends boolean = true>(x: number, y: number, isFloor?: T) => T extends true ? number : number[]` | Performs division without floating point errors | `division(0.1, 0.2); // 0.5` |
| factorial | `(x: number) => number` | Calculate factorial of a number | `factorial(5); // 120` |
| factorize | `(n: number) => number[]` | Prime factorization of a number | `factorize(12); // [2, 2, 3]` |
| flexibleNumberConversion | `(value: unknown) => number` | Flexible function to convert various inputs to numbers whenever possible | `flexibleNumberConversion("456"); // 456` |
| gcd | `(x: number, y: number, ...z: number[]) => number` | Greatest Common Divisor (GCD) | `gcd(12, 18); // 6` |
| getDecimalLength | `(value: number) => number` | Gets the number of decimal places in a number | `getDecimalLength(1.23); // 2` |
| lcm | `(x: number, y: number) => number` | Least Common Multiple (LCM) | `lcm(2, 3); // 6` |
| linearCongruentialGenerator | `(seed: number, max?: number, multiplier?: number, increment?: number) => number` | Linear Congruential Generator for random number generation | `linearCongruentialGenerator(12345);` |
| literalExpression | `(x: string) => string` | Solves literal equations with variables | `literalExpression("x+1=2"); // "1"` |
| mathConverter | `(equation: string) => string` | Expands square of n into a sum of simpler multiplications | `mathConverter("1250*1250"); // "1500*1000+400*100+200*100+50*50"` |
| mathSeparator | `(input: string \| number) => [number, number]` | Separates a number at its highest place value | `mathSeparator(1250); // [1000, 250]` |
| max | `(...number_: number[]) => number` | Returns the maximum value from the input numbers | `max(1, 2, 3); // 3` |
| median | `(array: number[]) => number` | Calculate the median of an array of numbers | `median([1, 3, 3, 6, 7, 8, 9]); // 6` |
| min | `(...number_: number[]) => number` | Returns the minimum value from the input numbers | `min(1, 2, 3); // 1` |
| mode | `(array: number[]) => number[]` | Finds the most frequently occurring value(s) in an array | `mode([1, 2, 2, 3, 3, 3]); // [3]` |
| multiples | `(x: number, n: number) => number[]` | Generate an array of multiples of a number | `multiples(2, 5); // [2, 4, 6, 8, 10]` |
| multiplication | `(...numbers: number[]) => number` | Performs multiplication without floating point errors for any number of arguments | `multiplication(0.1, 0.2, 0.3); // 0.006` |
| nCr | `(n: number, r: number) => number` | Calculates combinations (nCr) - number of ways to choose r items from n items | `nCr(5, 2); // 10` |
| nHr | `(n: number, r: number) => number` | Calculates combinations with repetition (nHr) | `nHr(5, 2); // 15` |
| nPr | `(n: number, r: number) => number` | Calculates permutations (nPr) - number of ways to arrange r items from n items | `nPr(5, 2); // 20` |
| percentile | `(array: number[], percentile: number) => number` | Calculate the nth percentile of values in an array | `percentile([1, 2, 3, 4, 5], 50); // 3` |
| primeFactorization | `(x: number) => Array<{number: number; count: number}>` | Performs prime factorization of a number | `primeFactorization(12); // [{number: 2, count: 2}, {number: 3, count: 1}]` |
| quotient | `(x: number, y: number) => number[]` | Computes quotient and remainder of division | `quotient(5, 2); // [2, 1]` |
| radToDeg | `(x: number) => number` | Converts radians to degrees | `radToDeg(Math.PI); // 180` |
| random | `(max: number, min?: number) => number` | Generates a random integer between min and max (inclusive) | `random(10); // returns number between 0 and 10` |
| reduce | `(x: number, y: number) => {x: number, y: number, gcd: number}` | Reduces a fraction to its lowest terms | `reduce(2, 4); // {x: 1, y: 2, gcd: 2}` |
| repeatedTrial | `(n: number, r: number, p: {x: number; y: number}) => number[]` | Calculate probability in repeated trials | `repeatedTrial(5, 2, {x: 1/3, y: 2/3}); // [10, 27]` |
| roundOf | `(value: number, precision?: number) => number` | Rounds a number to specified decimal places | `roundOf(1.234, 2); // 1.23` |
| solveEquation | `(coefficients: number[][], constants: number[]) => number[]` | Solves a system of linear equations using Gaussian elimination | `solveEquation([[1, 1], [1, 2]], [4, 10]); // [-2, 6]` |
| standardDeviation | `(values: number[]) => number` | Calculates the standard deviation of a set of values | `standardDeviation([1, 2, 3]); // 0.816496580927726` |
| subtract | `(...numbers: number[]) => number` | Performs subtraction with arbitrary number of arguments without floating point errors | `subtract(0.1, 0.2); // -0.1` |
| toBaseN | `(value: number, radix?: number) => string` | Converts a number to a string representation in the specified base | `toBaseN(10); // "1010" (binary)` |
| toCelsius | `(kelvin: number) => number` | Converts temperature from Kelvin to Celsius | `toCelsius(300); // 26.85` |
| toKelvin | `(celsius: number) => number` | Converts temperature from Celsius to Kelvin | `toKelvin(26.85); // 300` |
| uuidv7 | `() => string` | Generates a UUID v7 (Universally Unique Identifier version 7) | `uuidv7(); // e.g. "018d6e78-e1e5-7c3c-8bf9-ae5942f2ba1c"` |
| valueSwap | `(x: number, y: number) => [number, number]` | Swaps two numbers to ensure x < y | `valueSwap(2, 1); // [1, 2]` |
| xoshiro256 | `(state: [number, number, number, number], min?: number, max?: number) => number` | Generates random numbers using the Xoshiro256** algorithm | `xoshiro256([1, 2, 3, 4]); // random number between 0 and 1` |

### Object

| name | type | description | example |
|------|------|-------------|---------|
| has | `<T extends { [key: string]: unknown }>(object: T, path: string \| string[]) => boolean` | Determines if an object has a specified path | `has({ a: { b: 1 } }, "a.b"); // true` |
| isEmpty | `(object: Record<string, unknown>) => boolean` | Checks if an object is empty (has no own properties) | `isEmpty({}); // true` |
| keyBy | `<T>(collection: T[] \| Record<PropertyName, T>, iteratee?: Iteratee<T>) => Record<PropertyName, T>` | Creates an object composed of keys generated from the results of running each element of collection through iteratee | `keyBy([{id: 1, name: 'a'}, {id: 2, name: 'b'}], 'id'); // {1: {id: 1, name: 'a'}, 2: {id: 2, name: 'b'}}` |
| merge | `<T extends Record<string, unknown>>(target: T, ...sources: Partial<T>[]) => T` | Merges multiple objects into a single object (shallow merge) | `merge({a: 1}, {b: 2}); // {a: 1, b: 2}` |
| mergeDeep | `<T extends Record<string, unknown>>(target: T, ...sources: Partial<T>[]) => T` | Deeply merges multiple objects into a single object | `mergeDeep({a: {b: 1}}, {a: {c: 2}}); // {a: {b: 1, c: 2}}` |
| omit | `<T extends Record<string, unknown>, K extends keyof T>(object: T, ...keys: K[]) => Omit<T, K>` | Creates an object without the specified keys | `omit({a: 1, b: 2, c: 3}, 'b'); // {a: 1, c: 3}` |
| pick | `<T extends object, K extends keyof T>(object: T, ...keys: K[]) => Pick<T, K>` | Creates a new object with only the specified properties from the source object | `pick({ id: 1, name: 'Alice', age: 30 }, 'id', 'name'); // { id: 1, name: 'Alice' }` |
| pickDeep | `<T extends object, K extends PickDeepKey<T>>(object: T, ...keys: K[]) => PickDeep<T>` | Creates a new object by deeply selecting properties from the source object based on specified keys | `pickDeep({ a: { b: { c: 1, d: 2 }, e: 3 }, f: 4 }, 'a.b.c', 'f'); // { a: { b: { c: 1 } }, f: 4 }` |

### Simple

| name | type | description | example |
|------|------|-------------|---------|
| quickSortSimple | `<T>(array: T[], compareFunction?: CompareFunction<T>, startID?: number, endID?: number) => T[]` | Quick sort implementation for arrays | `quickSort([1, 3, 2, 4, 5], (a, b) => a - b); // [1, 2, 3, 4, 5]` |
| birthdaySimple | `(birthdays: Date \| string \| { year: number; mon: number; day: number }, timeDifference?: HoursTypeInt) => number` | Calculate age from birthdate | `birthdaySimple("2000-01-01");` |
| dayOfWeekSimple | `(properties?: Date \| string \| { year?: number; mon?: T; day?: DayTypeInt<T> }, timeDifference?: HoursTypeInt) => number` | Get day of the week | `dayOfWeekSimple("2000-01-01");` |
| nowSimple | `(timeDifference?: HoursTypeInt \| HoursType) => Date` | Get current date and time | `nowSimple(); // 2021-01-01T00:00:00.000Z` |
| deviationValueSimple | `(value: number, averageValue: number[] \| number, standardDeviationValue?: number) => number` | Calculate deviation score (T-score) | `deviationValueSimple(60, 50, 10); // 60` |

### String

| name | type | description | example |
|------|------|-------------|---------|
| camelCase | `(str: string) => string` | Converts a string to camelCase | `camelCase("hello-world"); // "helloWorld"` |
| deleteSpaces | `(string_: string) => string` | Removes all whitespace characters from a string | `deleteSpaces("Hello World"); // "HelloWorld"` |
| escapeHtml | `(str: string) => string` | Escapes HTML special characters in a string | `escapeHtml("<script>alert('XSS')</script>"); // "&lt;script&gt;alert(&#39;XSS&#39;)&lt;/script&gt;"` |
| formatString | `(template: string, ...values: unknown[]) => string` | Replaces placeholders in a template string with specified values | `formatString("Hello, {0}!", "World"); // "Hello, World!"` |
| fuzzySearch | `(query: string, items: string[], threshold?: number) => Array<{ item: string; score: number }>` | Perform fuzzy string matching on an array of strings | `fuzzySearch("hello", ["hello", "world", "helo", "help"]); // [{ item: "hello", score: 1 }, { item: "helo", score: 0.8 }, { item: "help", score: 0.6 }]` |
| fromBase64 | `(base64String: string) => string` | Converts Base64 to string | `fromBase64("SGVsbG8="); // "Hello"` |
| hasNoLetters | `(text: string) => boolean` | Checks if the string contains no letters (contains only emojis, numbers, or special characters) | `hasNoLetters("123"); // true` |
| kebabCase | `(str: string) => string` | Converts a string to kebab-case | `kebabCase("helloWorld"); // "hello-world"` |
| levenshteinDistance | `(string1: string, string2: string) => number` | Calculates the Levenshtein distance between two strings (minimum number of single-character edits) | `levenshteinDistance("kitten", "sitting"); // 3` |
| padEnd | `(string_: string, targetLength: number, padString: string) => string` | Adds the specified string to the end of the string until it reaches the specified length | `padEnd("123", 5, "0"); // "12300"` |
| padStart | `(string_: string, targetLength: number, padString: string) => string` | Pads the start of a string with another string until the target length is reached | `padStart("123", 5, "0"); // "00123"` |
| randomString | `(size?: number, char?: string) => string` | Generates a random string | `randomString(8); // "aB3dEf9h"` |
| randomStringInitialization | `(char?: string) => (size: number) => string` | Initializes a function that generates random strings | `const gen = randomStringInitialization("ABC"); gen(5); // "ABCAB"` |
| reverseString | `(char: string) => string` | Reverses a string | `reverseString("Hello"); // "olleH"` |
| slugify | `(str: string) => string` | Convert a string to a URL-friendly slug | `slugify("Hello World!"); // "hello-world"` |
| stringSimilarity | `(string1: string, string2: string) => number` | Calculates the similarity between two strings as a percentage (0-1) using Levenshtein distance | `stringSimilarity("hello", "hallo"); // 0.8` |
| toBase64 | `(char: string) => string` | Convert string to Base64 | `toBase64("Hello"); // "SGVsbG8="` |
| toHalfWidth | `(str: string) => string` | Convert full-width characters to half-width characters | `toHalfWidth("１２３ＡＢＣ"); // "123ABC"` |
| trimCharacters | `(string_: string, chars: string) => string` | Removes specified characters from both ends of a string | `trimCharacters("!!!hello!!!", "!"); // "hello"` |
| trimEndCharacters | `(string_: string, chars: string) => string` | Removes specified characters from the end of a string | `trimEndCharacters("hello!!!", "!"); // "hello"` |
| trimStartCharacters | `(string_: string, chars: string) => string` | Removes specified characters from the start of a string | `trimStartCharacters("!!!hello", "!"); // "hello"` |
| truncate | `(str: string, length: number, suffix?: string) => string` | Truncate a string to a specified length | `truncate("Hello World", 5); // "Hello..."` |
| unescapeHtml | `(str: string) => string` | Unescapes HTML entities in a string | `unescapeHtml("&lt;script&gt;alert(&quot;Hello&quot;);&lt;/script&gt;"); // "<script>alert("Hello");</script>"` |

### Time

| name | type | description | example |
|------|------|-------------|---------|
| convertTime | `(value: string \| number, fromUnit: TimeUnit \| TimeUnitShort, toUnit: TimeUnit \| TimeUnitShort) => number` | Converts time between different units | `convertTime(1, "hours", "minutes"); // 60` |
| normalizeTimeUnit | `(unit: TimeUnit \| TimeUnitShort, to: "long" \| "short") => TimeUnit \| TimeUnitShort` | Normalize time unit | `normalizeTimeUnit("h", "long"); // "hours"` |

### Tool

| name | type | description | example |
|------|------|-------------|---------|
| createPipeline | `<T>(initialValue: T) => Pipeline<T>` | Function that creates a Pipeline instance | `createPipeline(1)((x) => x + 1)(); // 2` |
| parseJson | `<T = unknown>(json: string) => T` | Parses a JSON string into a typed JavaScript value | `parseJson<{a: number}>('{"a": 1}'); // {a: 1}` |
| pipe | `<T>(initialValue: T) => Pipe<T>` | Creates a new Pipe instance with an initial value | `pipe(1).map(x => x + 1).end(); // 2` |

### UA

| name | type | description | example |
|------|------|-------------|---------|
| extractBrowserFromUserAgent | `(ua: string) => SimplifiedUserAgentInfoBrowser` | Extracts browser information from a User-Agent string | `extractBrowserFromUserAgent(navigator.userAgent); // "chrome"` |
| extractDeviceFromUserAgent | `(ua: string) => SimplifiedUserAgentInfoDevice` | Extracts device type information from a User-Agent string | `extractDeviceFromUserAgent(navigator.userAgent); // "desktop"` |
| extractOsFromUserAgent | `(ua: string) => SimplifiedUserAgentInfoOs` | Extracts operating system information from a User-Agent string | `extractOsFromUserAgent(navigator.userAgent); // "macos"` |
| parseUserAgent | `(userAgent: string) => SimplifiedUserAgentInfo` | Parse a User-Agent string to extract browser, device, and OS information | `parseUserAgent(navigator.userAgent); // {browser: "chrome", device: "desktop", os: "macos"}` |

### Unit

| name | type | description | example |
|------|------|-------------|---------|
| unitConverterInitialization | `<T extends { [k in K]: number }, K extends string \| number \| symbol>(toBaseUnitRatios: T) => (value: number, from: keyof T, to: keyof T) => number` | Unit converter initialization function | `const converter = unitConverterInitialization({meters: 1, kilometers: 1000}); converter(5, "kilometers", "meters"); // 5000` |

### Validate

| name | type | description | example |
|------|------|-------------|---------|
| array | `<A extends string \| number \| boolean, O extends {}>(option?: O, message?: string) => (values: A[]) => ValidateCoreReturnType<A[]>` | Creates an array validator with type-specific validation rules | `array()([1, 2, 3]); // {validate: true, message: "", type: [1, 2, 3]}` |
| boolean | `(message?: string) => (value: boolean) => ValidateCoreReturnType<boolean>` | Creates a boolean validator | `boolean()(true); // {validate: true, message: "", type: true}` |
| number | `<T extends ValidateReturnType<number>[]>(option?: T, message?: string) => (value: number) => ValidateCoreReturnType<number>` | Creates a number validator with optional validation rules | `number()(42); // {validate: true, message: "", type: 42}` |
| object | `<T extends {}>(option?: T, message?: string) => (value: Types<{}>) => ValidateCoreReturnType<{}>` | Creates an object validator with property-specific validation rules | `object({id: number()})({id: 1}); // {validate: true, message: "", type: {id: 1}}` |
| string | `<T extends ValidateReturnType<string>[]>(option?: T, message?: string) => (value: string) => ValidateCoreReturnType<string>` | Creates a string validator with optional validation rules | `string()("hello"); // {validate: true, message: "", type: "hello"}` |
| isArray | `<T>(array: unknown) => array is T[]` | Determines if the value is an array | `isArray([1, 2, 3]); // true` |
| isBrowser | `() => boolean` | Determines if the current environment is a browser | `isBrowser(); // true in browser` |
| isBun | `() => boolean` | Determines if the current environment is Bun runtime | `isBun(); // true in Bun` |
| isDeepEqual | `(a: unknown, b: unknown, options?: IsDeepEqualOptions) => boolean` | Performs deep equality comparison between two values with support for nested objects, arrays, Sets, Maps, and circular references | `isDeepEqual({ a: 1, b: [2, 3] }, { b: [2, 3], a: 1 }); // true` |
| isDictionaryObject | `<T extends { [key: string]: unknown }>(object: unknown) => object is T` | Determines if the value is a dictionary-type object | `isDictionaryObject({}); // true` |
| isDouble | `<T extends boolean = true>(x: unknown, loose?: T) => x is T extends true ? number \| string : number` | Determines if the value is a decimal number | `isDouble(0.1); // true` |
| isEqual | `(a: unknown, b: unknown) => boolean` | Evaluates true strict equality | `isEqual(1, 1); // true` |
| isNode | `() => boolean` | Determines if the current environment is Node.js | `isNode(); // true in Node.js` |
| isNodeWebkit | `() => boolean` | Determines if the current environment is Node-Webkit | `isNodeWebkit(); // true in Node-Webkit` |
| isNotEmpty | `(object: object) => boolean` | Checks if an object is not empty | `isNotEmpty({ a: 1 }); // true` |
| isNumber | `<T extends boolean>(number: unknown, loose?: T) => number is T extends true ? number \| string : number` | Determines if the value represents a number | `isNumber(0.1); // true` |
| isPerfectSquare | `(number_: number) => boolean` | Determines if a given integer is a perfect square | `isPerfectSquare(16); // true` |
| isPrimeNumber | `(n: number) => boolean` | Determines if a number is prime | `isPrimeNumber(17); // true` |
| isString | `(value: unknown) => value is string` | Determines if the value is a string | `isString("test"); // true` |
| isValueNaN | `(value: unknown, loose?: boolean) => boolean` | Determines if a value is NaN | `isValueNaN(parseInt("not a number")); // true` |

#### Validate Number Options

| name | type | description | example |
|------|------|-------------|---------|
| double | `(message?: string) => ValidateReturnType<number>` | Creates a validator for checking if a number is a floating point value | `number([double()])(-0.5); // valid` |
| even | `(message?: string) => ValidateReturnType<number>` | Creates a validator for checking if a number is even | `number([even()])(4); // valid` |
| maxValue | `(maxValue: number, message?: string) => ValidateReturnType<number>` | Creates a validator for checking if a number is less than or equal to a maximum value | `number([maxValue(100)])(50); // valid` |
| minValue | `(minValue: number, message?: string) => ValidateReturnType<number>` | Creates a validator for checking if a number is greater than or equal to a minimum value | `number([minValue(0)])(10); // valid` |
| odd | `(message?: string) => ValidateReturnType<number>` | Creates a validator for checking if a number is odd | `number([odd()])(3); // valid` |
| prime | `(message?: string) => ValidateReturnType<number>` | Creates a validator for checking if a number is prime | `number([prime()])(7); // valid` |

#### Validate String Options

| name | type | description | example |
|------|------|-------------|---------|
| email | `(message?: string) => ValidateReturnType<string>` | Creates a validator for checking if a string is a valid email address | `string([email()])("test@example.com"); // valid` |
| length_ | `(length: number, message?: string) => ValidateReturnType<string>` | Creates a validator for checking if a string has an exact length | `string([length_(5)])("hello"); // valid` |
| maxLength | `(maxLength: number, message?: string) => ValidateReturnType<string>` | Creates a validator for checking if a string's length is less than or equal to a maximum value | `string([maxLength(10)])("hello"); // valid` |
| minLength | `(minLength: number, message?: string) => ValidateReturnType<string>` | Creates a validator for checking if a string's length is greater than or equal to a minimum value | `string([minLength(3)])("hello"); // valid` |
| numberString | `(message?: string) => ValidateReturnType<string>` | Creates a validator for checking if a string represents a valid number | `string([numberString()])("123.45"); // valid` |
| regexMatch | `(pattern: RegExp, message?: string) => ValidateReturnType<string>` | Creates a validator for checking if a string matches a regular expression pattern | `string([regexMatch(/^[A-Z]/)])("Hello"); // valid` |
| uuid | `(versions?: number[], message?: string) => ValidateReturnType<string>` | Creates a validator for checking if a string is a valid UUID | `string([uuid()])("550e8400-e29b-41d4-a716-446655440000"); // valid` |

### Consts

| name | type | description | example |
|------|------|-------------|---------|
| OneSecondMs | `1000` | Number of milliseconds in one second | `OneSecondMs; // 1000` |
| OneMinuteMs | `60000` | Number of milliseconds in one minute | `OneMinuteMs; // 60000` |
| OneHourMs | `3600000` | Number of milliseconds in one hour | `OneHourMs; // 3600000` |
| OneDayMs | `86400000` | Number of milliseconds in one day | `OneDayMs; // 86400000` |
| OneWeekMs | `604800000` | Number of milliseconds in one week | `OneWeekMs; // 604800000` |
| OneMonthMs28 | `2419200000` | Number of milliseconds in one month (28 days) | `OneMonthMs28; // 2419200000` |
| OneMonthMs29 | `2505600000` | Number of milliseconds in one month (29 days) | `OneMonthMs29; // 2505600000` |
| OneMonthMs | `2592000000` | Number of milliseconds in one month (30 days) | `OneMonthMs; // 2592000000` |
| OneMonthMs31 | `2678400000` | Number of milliseconds in one month (31 days) | `OneMonthMs31; // 2678400000` |
| OneYearMs | `31536000000` | Number of milliseconds in one year (365 days) | `OneYearMs; // 31536000000` |
| OneYearMs366 | `31622400000` | Number of milliseconds in one year (366 days) | `OneYearMs366; // 31622400000` |
| HttpStatus | `Object` | All HTTP status codes | `HttpStatus.OK; // 200` |
| HttpInformationalStatus | `Object` | HTTP 1xx Informational Status Codes | `HttpInformationalStatus.CONTINUE; // 100` |
| HttpSuccessStatus | `Object` | HTTP 2xx Success Status Codes | `HttpSuccessStatus.OK; // 200` |
| HttpRedirectionStatus | `Object` | HTTP 3xx Redirection Status Codes | `HttpRedirectionStatus.MOVED_PERMANENTLY; // 301` |
| HttpClientErrorStatus | `Object` | HTTP 4xx Client Error Status Codes | `HttpClientErrorStatus.NOT_FOUND; // 404` |
| HttpServerErrorStatus | `Object` | HTTP 5xx Server Error Status Codes | `HttpServerErrorStatus.INTERNAL_SERVER_ERROR; // 500` |
