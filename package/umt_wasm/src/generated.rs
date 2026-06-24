#![allow(clippy::all)]
#![allow(clippy::pedantic)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(js_name = "generateNumberArray")]
pub fn array_generate_number_array(
    length: usize,
    min: Option<f64>,
    max: Option<f64>,
    random: bool,
) -> Vec<f64> {
    umt_rust::array::umt_generate_number_array(length, min, max, random)
}
#[wasm_bindgen(js_name = "generateNumberArrayI32")]
pub fn array_generate_number_array_i32(
    length: usize,
    min: Option<i32>,
    max: Option<i32>,
    random: bool,
) -> Vec<i32> {
    umt_rust::array::umt_generate_number_array_i32(length, min, max, random)
}
#[wasm_bindgen(js_name = "range")]
pub fn array_range(start: i32, end: Option<i32>, step: Option<i32>) -> Vec<i32> {
    umt_rust::array::umt_range(start, end, step)
}
#[wasm_bindgen(js_name = "rangeF64")]
pub fn array_range_f64(start: f64, end: Option<f64>, step: Option<f64>) -> Vec<f64> {
    umt_rust::array::umt_range_f64(start, end, step)
}
#[wasm_bindgen(js_name = "sum")]
pub fn array_sum(array: Vec<f64>) -> f64 {
    umt_rust::array::umt_sum(&array)
}
#[wasm_bindgen(js_name = "sumI64")]
pub fn array_sum_i64(array: Vec<i64>) -> i64 {
    umt_rust::array::umt_sum_i64(&array)
}
#[wasm_bindgen(js_name = "uniqueF64")]
pub fn array_unique_f64(array: Vec<f64>) -> Vec<f64> {
    umt_rust::array::umt_unique_f64(&array)
}
#[wasm_bindgen(js_name = "sleep")]
pub fn async_util_sleep(ms: u64) {
    umt_rust::async_util::umt_sleep(ms)
}
#[wasm_bindgen(js_name = "birthday")]
pub fn date_birthday(year: i32, month: u32, day: u32, time_difference: Option<i32>) -> u32 {
    umt_rust::date::umt_birthday(year, month, day, time_difference)
}
#[wasm_bindgen(js_name = "dayOfWeek")]
pub fn date_day_of_week(
    year: Option<i32>,
    month: Option<u32>,
    day: Option<u32>,
    time_difference: Option<i32>,
) -> Option<u32> {
    umt_rust::date::umt_day_of_week(year, month, day, time_difference)
}
#[wasm_bindgen(js_name = "getLocalTimezoneOffsetString")]
pub fn date_get_local_timezone_offset_string() -> String {
    umt_rust::date::umt_get_local_timezone_offset_string()
}
#[wasm_bindgen(js_name = "getTimezoneOffsetString")]
pub fn date_get_timezone_offset_string(offset_minutes: i32) -> String {
    umt_rust::date::umt_get_timezone_offset_string(offset_minutes)
}
#[wasm_bindgen(js_name = "getTimezoneOffsetStringCompact")]
pub fn date_get_timezone_offset_string_compact(offset_minutes: i32) -> String {
    umt_rust::date::umt_get_timezone_offset_string_compact(offset_minutes)
}
#[wasm_bindgen(js_name = "isLeapYear")]
pub fn date_is_leap_year(year: i32) -> bool {
    umt_rust::date::umt_is_leap_year(year)
}
#[wasm_bindgen(js_name = "todayDayOfWeek")]
pub fn date_today_day_of_week(time_difference: Option<i32>) -> u32 {
    umt_rust::date::umt_today_day_of_week(time_difference)
}
#[wasm_bindgen(js_name = "addition")]
pub fn math_addition(numbers: Vec<f64>) -> f64 {
    umt_rust::math::umt_addition(&numbers)
}
#[wasm_bindgen(js_name = "average")]
pub fn math_average(numbers: Vec<f64>) -> f64 {
    umt_rust::math::umt_average(numbers)
}
#[wasm_bindgen(js_name = "bitwiseLeft")]
pub fn math_bitwise_left(x: u32, k: i32) -> u32 {
    umt_rust::math::umt_bitwise_left(x, k)
}
#[wasm_bindgen(js_name = "clamp")]
pub fn math_clamp(value: f64, min: f64, max: f64) -> f64 {
    umt_rust::math::umt_clamp(value, min, max)
}
#[wasm_bindgen(js_name = "correlationCoefficient")]
pub fn math_correlation_coefficient(x: Vec<f64>, y: Vec<f64>) -> f64 {
    umt_rust::math::umt_correlation_coefficient(&x, &y)
}
#[wasm_bindgen(js_name = "degToRad")]
pub fn math_deg_to_rad(degrees: f64) -> f64 {
    umt_rust::math::umt_deg_to_rad(degrees)
}
#[wasm_bindgen(js_name = "deviationValue")]
pub fn math_deviation_value(value: f64, average_value: f64, standard_deviation_value: f64) -> f64 {
    umt_rust::math::umt_deviation_value(value, average_value, standard_deviation_value)
}
#[wasm_bindgen(js_name = "division")]
pub fn math_division(x: f64, y: f64) -> f64 {
    umt_rust::math::umt_division(x, y)
}
#[wasm_bindgen(js_name = "factorial")]
pub fn math_factorial(x: i32) -> i32 {
    umt_rust::math::umt_factorial(x)
}
#[wasm_bindgen(js_name = "factorize")]
pub fn math_factorize(n: i32) -> Vec<i32> {
    umt_rust::math::umt_factorize(n)
}
#[wasm_bindgen(js_name = "flexibleNumberConversion")]
pub fn math_flexible_number_conversion(value: &str) -> f64 {
    umt_rust::math::umt_flexible_number_conversion(value)
}
#[wasm_bindgen(js_name = "flexibleNumberConversionOpt")]
pub fn math_flexible_number_conversion_opt(value: Option<String>) -> f64 {
    umt_rust::math::umt_flexible_number_conversion_opt(value.as_deref())
}
#[wasm_bindgen(js_name = "gcd")]
pub fn math_gcd(x: i32, y: i32) -> i32 {
    umt_rust::math::umt_gcd(x, y)
}
#[wasm_bindgen(js_name = "gcdMany")]
pub fn math_gcd_many(numbers: Vec<i32>) -> i32 {
    umt_rust::math::umt_gcd_many(&numbers)
}
#[wasm_bindgen(js_name = "gcdMultiple")]
pub fn math_gcd_multiple(numbers: Vec<i32>) -> i32 {
    umt_rust::math::umt_gcd_multiple(&numbers)
}
#[wasm_bindgen(js_name = "getDecimalLength")]
pub fn math_get_decimal_length(value: f64) -> usize {
    umt_rust::math::umt_get_decimal_length(value)
}
#[wasm_bindgen(js_name = "inRange")]
pub fn math_in_range(value: f64, start: f64, end: f64) -> bool {
    umt_rust::math::umt_in_range(value, start, end)
}
#[wasm_bindgen(js_name = "lcm")]
pub fn math_lcm(a: i32, b: i32) -> i32 {
    umt_rust::math::umt_lcm(a, b)
}
#[wasm_bindgen(js_name = "mathConverter")]
pub fn math_math_converter(equation: &str) -> String {
    umt_rust::math::umt_math_converter(equation)
}
#[wasm_bindgen(js_name = "mathSeparator")]
pub fn math_math_separator(x: i32) -> Vec<i32> {
    umt_rust::math::umt_math_separator(x)
}
#[wasm_bindgen(js_name = "max")]
pub fn math_max(numbers: Vec<f64>) -> f64 {
    umt_rust::math::umt_max(&numbers)
}
#[wasm_bindgen(js_name = "median")]
pub fn math_median(array: Vec<f64>) -> f64 {
    umt_rust::math::umt_median(&array)
}
#[wasm_bindgen(js_name = "min")]
pub fn math_min(numbers: Vec<f64>) -> f64 {
    umt_rust::math::umt_min(&numbers)
}
#[wasm_bindgen(js_name = "mode")]
pub fn math_mode(array: Vec<f64>) -> Vec<f64> {
    umt_rust::math::umt_mode(&array)
}
#[wasm_bindgen(js_name = "multiples")]
pub fn math_multiples(x: f64, n: usize) -> Vec<f64> {
    umt_rust::math::umt_multiples(x, n)
}
#[wasm_bindgen(js_name = "multiplication")]
pub fn math_multiplication(numbers: Vec<f64>) -> f64 {
    umt_rust::math::umt_multiplication(&numbers)
}
#[wasm_bindgen(js_name = "ncr")]
pub fn math_ncr(n: i32, r: i32) -> i32 {
    umt_rust::math::umt_ncr(n, r)
}
#[wasm_bindgen(js_name = "nhr")]
pub fn math_nhr(n: i32, r: i32) -> i32 {
    umt_rust::math::umt_nhr(n, r)
}
#[wasm_bindgen(js_name = "npr")]
pub fn math_npr(n: i32, r: i32) -> i32 {
    umt_rust::math::umt_npr(n, r)
}
#[wasm_bindgen(js_name = "percentile")]
pub fn math_percentile(array: Vec<f64>, percentile: f64) -> Result<f64, JsValue> {
    umt_rust::math::umt_percentile(&array, percentile)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
#[wasm_bindgen(js_name = "radToDeg")]
pub fn math_rad_to_deg(x: f64) -> f64 {
    umt_rust::math::umt_rad_to_deg(x)
}
#[wasm_bindgen(js_name = "random")]
pub fn math_random(max: i64, min: i64) -> i64 {
    umt_rust::math::umt_random(max, min)
}
#[wasm_bindgen(js_name = "randomMax")]
pub fn math_random_max(max: i64) -> i64 {
    umt_rust::math::umt_random_max(max)
}
#[wasm_bindgen(js_name = "roundOf")]
pub fn math_round_of(num: f64, precision: i32) -> f64 {
    umt_rust::math::umt_round_of(num, precision)
}
#[wasm_bindgen(js_name = "softmax")]
pub fn math_softmax(x: Vec<f64>, decimal_place: i32) -> Vec<f64> {
    umt_rust::math::umt_softmax(x, decimal_place)
}
#[wasm_bindgen(js_name = "standardDeviation")]
pub fn math_standard_deviation(values: Vec<f64>) -> f64 {
    umt_rust::math::umt_standard_deviation(&values)
}
#[wasm_bindgen(js_name = "subtract")]
pub fn math_subtract(numbers: Vec<f64>) -> f64 {
    umt_rust::math::umt_subtract(&numbers)
}
#[wasm_bindgen(js_name = "sumPrecise")]
pub fn math_sum_precise(numbers: Vec<f64>) -> f64 {
    umt_rust::math::umt_sum_precise(&numbers)
}
#[wasm_bindgen(js_name = "toBaseN")]
pub fn math_to_base_n(value: i64, radix: u32) -> String {
    umt_rust::math::umt_to_base_n(value, radix)
}
#[wasm_bindgen(js_name = "toBinary")]
pub fn math_to_binary(value: i64) -> String {
    umt_rust::math::umt_to_binary(value)
}
#[wasm_bindgen(js_name = "toCelsius")]
pub fn math_to_celsius(kelvin: f64) -> f64 {
    umt_rust::math::umt_to_celsius(kelvin)
}
#[wasm_bindgen(js_name = "toHex")]
pub fn math_to_hex(value: i64) -> String {
    umt_rust::math::umt_to_hex(value)
}
#[wasm_bindgen(js_name = "toKelvin")]
pub fn math_to_kelvin(celsius: f64) -> f64 {
    umt_rust::math::umt_to_kelvin(celsius)
}
#[wasm_bindgen(js_name = "uuidv7")]
pub fn math_uuidv7() -> String {
    umt_rust::math::umt_uuidv7()
}
#[wasm_bindgen(js_name = "valueSwap")]
pub fn math_value_swap(x: f64, y: f64) -> Vec<f64> {
    umt_rust::math::umt_value_swap(x, y)
}
#[wasm_bindgen(js_name = "literalExpression")]
pub fn math_calculator_literal_expression(x: &str) -> String {
    umt_rust::math::calculator::umt_literal_expression(x)
}
#[wasm_bindgen(js_name = "formatNumberDefault")]
pub fn number_format_number_default(value: f64) -> String {
    umt_rust::number::umt_format_number_default(value)
}
#[wasm_bindgen(js_name = "toOrdinal")]
pub fn number_to_ordinal(value: f64) -> String {
    umt_rust::number::umt_to_ordinal(value)
}
#[wasm_bindgen(js_name = "toPercentage")]
pub fn number_to_percentage(value: f64, total: f64, decimals: i32) -> f64 {
    umt_rust::number::umt_to_percentage(value, total, decimals)
}
#[wasm_bindgen(js_name = "toPercentageDefault")]
pub fn number_to_percentage_default(value: f64, total: f64) -> f64 {
    umt_rust::number::umt_to_percentage_default(value, total)
}
#[wasm_bindgen(js_name = "pathSegments")]
pub fn object_path_segments(path: &str) -> Vec<String> {
    umt_rust::object::umt_path_segments(path)
}
#[wasm_bindgen(js_name = "randomBoolean")]
pub fn random_random_boolean(probability: f64) -> bool {
    umt_rust::random::umt_random_boolean(probability)
}
#[wasm_bindgen(js_name = "randomBooleanDefault")]
pub fn random_random_boolean_default() -> bool {
    umt_rust::random::umt_random_boolean_default()
}
#[wasm_bindgen(js_name = "randomFloat")]
pub fn random_random_float(min: f64, max: f64) -> f64 {
    umt_rust::random::umt_random_float(min, max)
}
#[wasm_bindgen(js_name = "randomInt")]
pub fn random_random_int(min: f64, max: f64) -> f64 {
    umt_rust::random::umt_random_int(min, max)
}
#[wasm_bindgen(js_name = "randomUuid")]
pub fn random_random_uuid() -> String {
    umt_rust::random::umt_random_uuid()
}
#[wasm_bindgen(js_name = "simpleBirthday")]
pub fn simple_birthday(year: i32, mon: u32, day: u32, time_difference: i32) -> i32 {
    umt_rust::simple::umt_birthday(year, mon, day, time_difference)
}
#[wasm_bindgen(js_name = "birthdaySimple")]
pub fn simple_birthday_simple(birthdays: &str) -> i32 {
    umt_rust::simple::umt_birthday_simple(birthdays)
}
#[wasm_bindgen(js_name = "birthdaySimpleStr")]
pub fn simple_birthday_simple_str(birthdays: &str, time_difference: i32) -> i32 {
    umt_rust::simple::umt_birthday_simple_str(birthdays, time_difference)
}
#[wasm_bindgen(js_name = "dayOfWeekSimpleStr")]
pub fn simple_day_of_week_simple_str(date_str: &str, time_difference: i32) -> u32 {
    umt_rust::simple::umt_day_of_week_simple_str(date_str, time_difference)
}
#[wasm_bindgen(js_name = "deviationValueSimple")]
pub fn simple_deviation_value_simple(
    value: f64,
    average_value: f64,
    standard_deviation_value: f64,
) -> f64 {
    umt_rust::simple::umt_deviation_value_simple(value, average_value, standard_deviation_value)
}
#[wasm_bindgen(js_name = "deviationValueSimpleFromArray")]
pub fn simple_deviation_value_simple_from_array(value: f64, reference_values: Vec<f64>) -> f64 {
    umt_rust::simple::umt_deviation_value_simple_from_array(value, reference_values)
}
#[wasm_bindgen(js_name = "quickSortSimpleF64")]
pub fn simple_quick_sort_simple_f64(
    array: Vec<f64>,
    ascending: bool,
    start_id: Option<isize>,
    end_id: Option<isize>,
) -> Vec<f64> {
    umt_rust::simple::umt_quick_sort_simple_f64(&array, ascending, start_id, end_id)
}
#[wasm_bindgen(js_name = "quickSortSimpleI32")]
pub fn simple_quick_sort_simple_i32(
    array: Vec<i32>,
    ascending: bool,
    start_id: Option<isize>,
    end_id: Option<isize>,
) -> Vec<i32> {
    umt_rust::simple::umt_quick_sort_simple_i32(&array, ascending, start_id, end_id)
}
#[wasm_bindgen(js_name = "camelCase")]
pub fn string_camel_case(s: &str) -> String {
    umt_rust::string::umt_camel_case(s)
}
#[wasm_bindgen(js_name = "capitalize")]
pub fn string_capitalize(s: &str) -> String {
    umt_rust::string::umt_capitalize(s)
}
#[wasm_bindgen(js_name = "capitalizeWord")]
pub fn string_capitalize_word(word: &str) -> String {
    umt_rust::string::umt_capitalize_word(word)
}
#[wasm_bindgen(js_name = "constantCase")]
pub fn string_constant_case(s: &str) -> String {
    umt_rust::string::umt_constant_case(s)
}
#[wasm_bindgen(js_name = "countOccurrences")]
pub fn string_count_occurrences(s: &str, search: &str) -> usize {
    umt_rust::string::umt_count_occurrences(s, search)
}
#[wasm_bindgen(js_name = "deburr")]
pub fn string_deburr(s: &str) -> String {
    umt_rust::string::umt_deburr(s)
}
#[wasm_bindgen(js_name = "dedent")]
pub fn string_dedent(s: &str) -> String {
    umt_rust::string::umt_dedent(s)
}
#[wasm_bindgen(js_name = "deleteSpaces")]
pub fn string_delete_spaces(s: &str) -> String {
    umt_rust::string::umt_delete_spaces(s)
}
#[wasm_bindgen(js_name = "ensurePrefix")]
pub fn string_ensure_prefix(s: &str, prefix: &str) -> String {
    umt_rust::string::umt_ensure_prefix(s, prefix)
}
#[wasm_bindgen(js_name = "ensureSuffix")]
pub fn string_ensure_suffix(s: &str, suffix: &str) -> String {
    umt_rust::string::umt_ensure_suffix(s, suffix)
}
#[wasm_bindgen(js_name = "escapeHtml")]
pub fn string_escape_html(s: &str) -> String {
    umt_rust::string::umt_escape_html(s)
}
#[wasm_bindgen(js_name = "formatStringIndexed")]
pub fn string_format_string_indexed(template: &str, values: Vec<String>) -> String {
    let values_refs: Vec<&str> = values.iter().map(|s| s.as_str()).collect();
    umt_rust::string::umt_format_string_indexed(template, &values_refs)
}
#[wasm_bindgen(js_name = "fromBase64")]
pub fn string_from_base64(base64_string: &str) -> Result<String, JsValue> {
    umt_rust::string::umt_from_base64(base64_string).map_err(|e| JsValue::from_str(&e.to_string()))
}
#[wasm_bindgen(js_name = "hasNoLetters")]
pub fn string_has_no_letters(text: &str) -> bool {
    umt_rust::string::umt_has_no_letters(text)
}
#[wasm_bindgen(js_name = "kebabCase")]
pub fn string_kebab_case(s: &str) -> String {
    umt_rust::string::umt_kebab_case(s)
}
#[wasm_bindgen(js_name = "levenshteinDistance")]
pub fn string_levenshtein_distance(s1: &str, s2: &str) -> usize {
    umt_rust::string::umt_levenshtein_distance(s1, s2)
}
#[wasm_bindgen(js_name = "normalizeWhitespace")]
pub fn string_normalize_whitespace(s: &str) -> String {
    umt_rust::string::umt_normalize_whitespace(s)
}
#[wasm_bindgen(js_name = "padEnd")]
pub fn string_pad_end(s: &str, target_length: usize, pad_string: &str) -> String {
    umt_rust::string::umt_pad_end(s, target_length, pad_string)
}
#[wasm_bindgen(js_name = "padStart")]
pub fn string_pad_start(s: &str, target_length: usize, pad_string: &str) -> String {
    umt_rust::string::umt_pad_start(s, target_length, pad_string)
}
#[wasm_bindgen(js_name = "pascalCase")]
pub fn string_pascal_case(s: &str) -> String {
    umt_rust::string::umt_pascal_case(s)
}
#[wasm_bindgen(js_name = "randomString")]
pub fn string_random_string(size: usize, chars: Option<String>) -> String {
    umt_rust::string::umt_random_string(size, chars.as_deref())
}
#[wasm_bindgen(js_name = "randomStringDefault")]
pub fn string_random_string_default(size: usize) -> String {
    umt_rust::string::umt_random_string_default(size)
}
#[wasm_bindgen(js_name = "removePrefix")]
pub fn string_remove_prefix(s: &str, prefix: &str) -> String {
    umt_rust::string::umt_remove_prefix(s, prefix)
}
#[wasm_bindgen(js_name = "removeSuffix")]
pub fn string_remove_suffix(s: &str, suffix: &str) -> String {
    umt_rust::string::umt_remove_suffix(s, suffix)
}
#[wasm_bindgen(js_name = "reverseString")]
pub fn string_reverse_string(s: &str) -> String {
    umt_rust::string::umt_reverse_string(s)
}
#[wasm_bindgen(js_name = "sanitizeString")]
pub fn string_sanitize_string(s: &str) -> String {
    umt_rust::string::umt_sanitize_string(s)
}
#[wasm_bindgen(js_name = "slugify")]
pub fn string_slugify(s: &str) -> String {
    umt_rust::string::umt_slugify(s)
}
#[wasm_bindgen(js_name = "snakeCase")]
pub fn string_snake_case(s: &str) -> String {
    umt_rust::string::umt_snake_case(s)
}
#[wasm_bindgen(js_name = "splitByLength")]
pub fn string_split_by_length(s: &str, length: usize) -> Vec<String> {
    umt_rust::string::umt_split_by_length(s, length)
}
#[wasm_bindgen(js_name = "stringSimilarity")]
pub fn string_string_similarity(s1: &str, s2: &str) -> f64 {
    umt_rust::string::umt_string_similarity(s1, s2)
}
#[wasm_bindgen(js_name = "stripAnsi")]
pub fn string_strip_ansi(s: &str) -> String {
    umt_rust::string::umt_strip_ansi(s)
}
#[wasm_bindgen(js_name = "stripTags")]
pub fn string_strip_tags(s: &str) -> String {
    umt_rust::string::umt_strip_tags(s)
}
#[wasm_bindgen(js_name = "swapCase")]
pub fn string_swap_case(s: &str) -> String {
    umt_rust::string::umt_swap_case(s)
}
#[wasm_bindgen(js_name = "titleCase")]
pub fn string_title_case(s: &str) -> String {
    umt_rust::string::umt_title_case(s)
}
#[wasm_bindgen(js_name = "toBase64")]
pub fn string_to_base64(s: &str) -> String {
    umt_rust::string::umt_to_base64(s)
}
#[wasm_bindgen(js_name = "toFullWidth")]
pub fn string_to_full_width(s: &str) -> String {
    umt_rust::string::umt_to_full_width(s)
}
#[wasm_bindgen(js_name = "toHalfWidth")]
pub fn string_to_half_width(s: &str) -> String {
    umt_rust::string::umt_to_half_width(s)
}
#[wasm_bindgen(js_name = "trimCharacters")]
pub fn string_trim_characters(s: &str, chars: &str) -> String {
    umt_rust::string::umt_trim_characters(s, chars)
}
#[wasm_bindgen(js_name = "trimEndCharacters")]
pub fn string_trim_end_characters(s: &str, chars: &str) -> String {
    umt_rust::string::umt_trim_end_characters(s, chars)
}
#[wasm_bindgen(js_name = "trimStartCharacters")]
pub fn string_trim_start_characters(s: &str, chars: &str) -> String {
    umt_rust::string::umt_trim_start_characters(s, chars)
}
#[wasm_bindgen(js_name = "truncate")]
pub fn string_truncate(s: &str, length: usize, suffix: &str) -> String {
    umt_rust::string::umt_truncate(s, length, suffix)
}
#[wasm_bindgen(js_name = "truncateDefault")]
pub fn string_truncate_default(s: &str, length: usize) -> String {
    umt_rust::string::umt_truncate_default(s, length)
}
#[wasm_bindgen(js_name = "uncapitalize")]
pub fn string_uncapitalize(s: &str) -> String {
    umt_rust::string::umt_uncapitalize(s)
}
#[wasm_bindgen(js_name = "unescapeHtml")]
pub fn string_unescape_html(s: &str) -> String {
    umt_rust::string::umt_unescape_html(s)
}
#[wasm_bindgen(js_name = "wordCount")]
pub fn string_word_count(s: &str) -> usize {
    umt_rust::string::umt_word_count(s)
}
#[wasm_bindgen(js_name = "convertTime")]
pub fn time_convert_time(value: f64, from_unit: &str, to_unit: &str) -> Option<f64> {
    umt_rust::time::umt_convert_time(value, from_unit, to_unit)
}
#[wasm_bindgen(js_name = "convertTimeFromStr")]
pub fn time_convert_time_from_str(value: &str, from_unit: &str, to_unit: &str) -> Option<f64> {
    umt_rust::time::umt_convert_time_from_str(value, from_unit, to_unit)
}
#[wasm_bindgen(js_name = "escapeRegexp")]
pub fn tool_escape_regexp(string: &str) -> String {
    umt_rust::tool::umt_escape_regexp(string)
}
#[wasm_bindgen(js_name = "isAbsoluteUrl")]
pub fn url_is_absolute_url(url: &str) -> bool {
    umt_rust::url::umt_is_absolute_url(url)
}
#[wasm_bindgen(js_name = "joinPath")]
pub fn url_join_path(segments: Vec<String>) -> String {
    let segments_refs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();
    umt_rust::url::umt_join_path(&segments_refs)
}
#[wasm_bindgen(js_name = "isBrowser")]
pub fn validate_is_browser() -> bool {
    umt_rust::validate::umt_is_browser()
}
#[wasm_bindgen(js_name = "isBun")]
pub fn validate_is_bun() -> bool {
    umt_rust::validate::umt_is_bun()
}
#[wasm_bindgen(js_name = "isDouble")]
pub fn validate_is_double(x: f64) -> bool {
    umt_rust::validate::umt_is_double(x)
}
#[wasm_bindgen(js_name = "isDoubleStr")]
pub fn validate_is_double_str(s: &str) -> bool {
    umt_rust::validate::umt_is_double_str(s)
}
#[wasm_bindgen(js_name = "isEqualF64")]
pub fn validate_is_equal_f64(a: f64, b: f64) -> bool {
    umt_rust::validate::umt_is_equal_f64(a, b)
}
#[wasm_bindgen(js_name = "isNode")]
pub fn validate_is_node() -> bool {
    umt_rust::validate::umt_is_node()
}
#[wasm_bindgen(js_name = "isNodeWebkit")]
pub fn validate_is_node_webkit() -> bool {
    umt_rust::validate::umt_is_node_webkit()
}
#[wasm_bindgen(js_name = "isNotEmptyStr")]
pub fn validate_is_not_empty_str(s: &str) -> bool {
    umt_rust::validate::umt_is_not_empty_str(s)
}
#[wasm_bindgen(js_name = "isNumber")]
pub fn validate_is_number(number: f64) -> bool {
    umt_rust::validate::umt_is_number(number)
}
#[wasm_bindgen(js_name = "isNumberI64")]
pub fn validate_is_number_i64(_number: i64) -> bool {
    umt_rust::validate::umt_is_number_i64(_number)
}
#[wasm_bindgen(js_name = "isNumberStr")]
pub fn validate_is_number_str(s: &str) -> bool {
    umt_rust::validate::umt_is_number_str(s)
}
#[wasm_bindgen(js_name = "isPerfectSquare")]
pub fn validate_is_perfect_square(number: i64) -> bool {
    umt_rust::validate::umt_is_perfect_square(number)
}
#[wasm_bindgen(js_name = "isPerfectSquareF64")]
pub fn validate_is_perfect_square_f64(number: f64) -> bool {
    umt_rust::validate::umt_is_perfect_square_f64(number)
}
#[wasm_bindgen(js_name = "isPrimeNumber")]
pub fn validate_is_prime_number(n: i64) -> bool {
    umt_rust::validate::umt_is_prime_number(n)
}
#[wasm_bindgen(js_name = "isPrimeNumberUsize")]
pub fn validate_is_prime_number_usize(n: usize) -> bool {
    umt_rust::validate::umt_is_prime_number_usize(n)
}
#[wasm_bindgen(js_name = "isString")]
pub fn validate_is_string(_value: &str) -> bool {
    umt_rust::validate::umt_is_string(_value)
}
#[wasm_bindgen(js_name = "isValueNan")]
pub fn validate_is_value_nan(value: f64) -> bool {
    umt_rust::validate::umt_is_value_nan(value)
}
#[wasm_bindgen(js_name = "isValueNanStr")]
pub fn validate_is_value_nan_str(value: &str, loose: bool) -> bool {
    umt_rust::validate::umt_is_value_nan_str(value, loose)
}
#[wasm_bindgen(js_name = "isValueNanStrLoose")]
pub fn validate_is_value_nan_str_loose(value: &str) -> bool {
    umt_rust::validate::umt_is_value_nan_str_loose(value)
}
#[wasm_bindgen(js_name = "isValueNanStrStrict")]
pub fn validate_is_value_nan_str_strict(_value: &str) -> bool {
    umt_rust::validate::umt_is_value_nan_str_strict(_value)
}
#[wasm_bindgen(js_name = "validateEmail")]
pub fn validate_validate_email(email: &str) -> bool {
    umt_rust::validate::umt_validate_email(email)
}
