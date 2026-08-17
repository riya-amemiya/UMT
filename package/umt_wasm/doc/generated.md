# Generated wasm bindings

This file is produced by `codegen/`. Do not edit by hand.

Generated 150 wrappers. 274 functions are not auto-exposable and are listed below for future hand-written adapters.

## Generated

| JS name | Rust source |
|---|---|
| `generateNumberArray` | `umt_rust::array::umt_generate_number_array` |
| `generateNumberArrayI32` | `umt_rust::array::umt_generate_number_array_i32` |
| `range` | `umt_rust::array::umt_range` |
| `rangeF64` | `umt_rust::array::umt_range_f64` |
| `sum` | `umt_rust::array::umt_sum` |
| `sumI64` | `umt_rust::array::umt_sum_i64` |
| `uniqueF64` | `umt_rust::array::umt_unique_f64` |
| `sleep` | `umt_rust::async_util::umt_sleep` |
| `birthday` | `umt_rust::date::umt_birthday` |
| `dayOfWeek` | `umt_rust::date::umt_day_of_week` |
| `getLocalTimezoneOffsetString` | `umt_rust::date::umt_get_local_timezone_offset_string` |
| `getTimezoneOffsetString` | `umt_rust::date::umt_get_timezone_offset_string` |
| `getTimezoneOffsetStringCompact` | `umt_rust::date::umt_get_timezone_offset_string_compact` |
| `isLeapYear` | `umt_rust::date::umt_is_leap_year` |
| `todayDayOfWeek` | `umt_rust::date::umt_today_day_of_week` |
| `addition` | `umt_rust::math::umt_addition` |
| `average` | `umt_rust::math::umt_average` |
| `bitwiseLeft` | `umt_rust::math::umt_bitwise_left` |
| `clamp` | `umt_rust::math::umt_clamp` |
| `correlationCoefficient` | `umt_rust::math::umt_correlation_coefficient` |
| `degToRad` | `umt_rust::math::umt_deg_to_rad` |
| `deviationValue` | `umt_rust::math::umt_deviation_value` |
| `division` | `umt_rust::math::umt_division` |
| `factorial` | `umt_rust::math::umt_factorial` |
| `factorize` | `umt_rust::math::umt_factorize` |
| `flexibleNumberConversion` | `umt_rust::math::umt_flexible_number_conversion` |
| `flexibleNumberConversionOpt` | `umt_rust::math::umt_flexible_number_conversion_opt` |
| `gcd` | `umt_rust::math::umt_gcd` |
| `gcdMany` | `umt_rust::math::umt_gcd_many` |
| `gcdMultiple` | `umt_rust::math::umt_gcd_multiple` |
| `getDecimalLength` | `umt_rust::math::umt_get_decimal_length` |
| `inRange` | `umt_rust::math::umt_in_range` |
| `lcm` | `umt_rust::math::umt_lcm` |
| `mathConverter` | `umt_rust::math::umt_math_converter` |
| `mathSeparator` | `umt_rust::math::umt_math_separator` |
| `max` | `umt_rust::math::umt_max` |
| `median` | `umt_rust::math::umt_median` |
| `min` | `umt_rust::math::umt_min` |
| `mode` | `umt_rust::math::umt_mode` |
| `multiples` | `umt_rust::math::umt_multiples` |
| `multiplication` | `umt_rust::math::umt_multiplication` |
| `ncr` | `umt_rust::math::umt_ncr` |
| `nhr` | `umt_rust::math::umt_nhr` |
| `npr` | `umt_rust::math::umt_npr` |
| `percentile` | `umt_rust::math::umt_percentile` |
| `radToDeg` | `umt_rust::math::umt_rad_to_deg` |
| `random` | `umt_rust::math::umt_random` |
| `randomMax` | `umt_rust::math::umt_random_max` |
| `roundOf` | `umt_rust::math::umt_round_of` |
| `softmax` | `umt_rust::math::umt_softmax` |
| `standardDeviation` | `umt_rust::math::umt_standard_deviation` |
| `subtract` | `umt_rust::math::umt_subtract` |
| `sumPrecise` | `umt_rust::math::umt_sum_precise` |
| `toBaseN` | `umt_rust::math::umt_to_base_n` |
| `toBinary` | `umt_rust::math::umt_to_binary` |
| `toCelsius` | `umt_rust::math::umt_to_celsius` |
| `toHex` | `umt_rust::math::umt_to_hex` |
| `toKelvin` | `umt_rust::math::umt_to_kelvin` |
| `uuidv7` | `umt_rust::math::umt_uuidv7` |
| `valueSwap` | `umt_rust::math::umt_value_swap` |
| `literalExpression` | `umt_rust::math::calculator::umt_literal_expression` |
| `formatNumberDefault` | `umt_rust::number::umt_format_number_default` |
| `toOrdinal` | `umt_rust::number::umt_to_ordinal` |
| `toPercentage` | `umt_rust::number::umt_to_percentage` |
| `toPercentageDefault` | `umt_rust::number::umt_to_percentage_default` |
| `pathSegments` | `umt_rust::object::umt_path_segments` |
| `randomBoolean` | `umt_rust::random::umt_random_boolean` |
| `randomBooleanDefault` | `umt_rust::random::umt_random_boolean_default` |
| `randomFloat` | `umt_rust::random::umt_random_float` |
| `randomInt` | `umt_rust::random::umt_random_int` |
| `randomUuid` | `umt_rust::random::umt_random_uuid` |
| `simpleBirthday` | `umt_rust::simple::umt_birthday` |
| `birthdaySimple` | `umt_rust::simple::umt_birthday_simple` |
| `birthdaySimpleStr` | `umt_rust::simple::umt_birthday_simple_str` |
| `dayOfWeekSimpleStr` | `umt_rust::simple::umt_day_of_week_simple_str` |
| `deviationValueSimple` | `umt_rust::simple::umt_deviation_value_simple` |
| `deviationValueSimpleFromArray` | `umt_rust::simple::umt_deviation_value_simple_from_array` |
| `quickSortSimpleF64` | `umt_rust::simple::umt_quick_sort_simple_f64` |
| `quickSortSimpleI32` | `umt_rust::simple::umt_quick_sort_simple_i32` |
| `camelCase` | `umt_rust::string::umt_camel_case` |
| `capitalize` | `umt_rust::string::umt_capitalize` |
| `capitalizeWord` | `umt_rust::string::umt_capitalize_word` |
| `constantCase` | `umt_rust::string::umt_constant_case` |
| `countOccurrences` | `umt_rust::string::umt_count_occurrences` |
| `deburr` | `umt_rust::string::umt_deburr` |
| `dedent` | `umt_rust::string::umt_dedent` |
| `deleteSpaces` | `umt_rust::string::umt_delete_spaces` |
| `ensurePrefix` | `umt_rust::string::umt_ensure_prefix` |
| `ensureSuffix` | `umt_rust::string::umt_ensure_suffix` |
| `escapeHtml` | `umt_rust::string::umt_escape_html` |
| `formatStringIndexed` | `umt_rust::string::umt_format_string_indexed` |
| `fromBase64` | `umt_rust::string::umt_from_base64` |
| `hasNoLetters` | `umt_rust::string::umt_has_no_letters` |
| `kebabCase` | `umt_rust::string::umt_kebab_case` |
| `levenshteinDistance` | `umt_rust::string::umt_levenshtein_distance` |
| `normalizeWhitespace` | `umt_rust::string::umt_normalize_whitespace` |
| `padEnd` | `umt_rust::string::umt_pad_end` |
| `padStart` | `umt_rust::string::umt_pad_start` |
| `pascalCase` | `umt_rust::string::umt_pascal_case` |
| `randomString` | `umt_rust::string::umt_random_string` |
| `randomStringDefault` | `umt_rust::string::umt_random_string_default` |
| `removePrefix` | `umt_rust::string::umt_remove_prefix` |
| `removeSuffix` | `umt_rust::string::umt_remove_suffix` |
| `reverseString` | `umt_rust::string::umt_reverse_string` |
| `sanitizeString` | `umt_rust::string::umt_sanitize_string` |
| `slugify` | `umt_rust::string::umt_slugify` |
| `snakeCase` | `umt_rust::string::umt_snake_case` |
| `splitByLength` | `umt_rust::string::umt_split_by_length` |
| `stringSimilarity` | `umt_rust::string::umt_string_similarity` |
| `stripAnsi` | `umt_rust::string::umt_strip_ansi` |
| `stripTags` | `umt_rust::string::umt_strip_tags` |
| `swapCase` | `umt_rust::string::umt_swap_case` |
| `titleCase` | `umt_rust::string::umt_title_case` |
| `toBase64` | `umt_rust::string::umt_to_base64` |
| `toFullWidth` | `umt_rust::string::umt_to_full_width` |
| `toHalfWidth` | `umt_rust::string::umt_to_half_width` |
| `trimCharacters` | `umt_rust::string::umt_trim_characters` |
| `trimEndCharacters` | `umt_rust::string::umt_trim_end_characters` |
| `trimStartCharacters` | `umt_rust::string::umt_trim_start_characters` |
| `truncate` | `umt_rust::string::umt_truncate` |
| `truncateDefault` | `umt_rust::string::umt_truncate_default` |
| `uncapitalize` | `umt_rust::string::umt_uncapitalize` |
| `unescapeHtml` | `umt_rust::string::umt_unescape_html` |
| `wordCount` | `umt_rust::string::umt_word_count` |
| `convertTime` | `umt_rust::time::umt_convert_time` |
| `convertTimeFromStr` | `umt_rust::time::umt_convert_time_from_str` |
| `escapeRegexp` | `umt_rust::tool::umt_escape_regexp` |
| `isAbsoluteUrl` | `umt_rust::url::umt_is_absolute_url` |
| `joinPath` | `umt_rust::url::umt_join_path` |
| `isBrowser` | `umt_rust::validate::umt_is_browser` |
| `isBun` | `umt_rust::validate::umt_is_bun` |
| `isDouble` | `umt_rust::validate::umt_is_double` |
| `isDoubleStr` | `umt_rust::validate::umt_is_double_str` |
| `isEqualF64` | `umt_rust::validate::umt_is_equal_f64` |
| `isNode` | `umt_rust::validate::umt_is_node` |
| `isNodeWebkit` | `umt_rust::validate::umt_is_node_webkit` |
| `isNotEmptyStr` | `umt_rust::validate::umt_is_not_empty_str` |
| `isNumber` | `umt_rust::validate::umt_is_number` |
| `isNumberI64` | `umt_rust::validate::umt_is_number_i64` |
| `isNumberStr` | `umt_rust::validate::umt_is_number_str` |
| `isPerfectSquare` | `umt_rust::validate::umt_is_perfect_square` |
| `isPerfectSquareF64` | `umt_rust::validate::umt_is_perfect_square_f64` |
| `isPrimeNumber` | `umt_rust::validate::umt_is_prime_number` |
| `isPrimeNumberUsize` | `umt_rust::validate::umt_is_prime_number_usize` |
| `isString` | `umt_rust::validate::umt_is_string` |
| `isValueNan` | `umt_rust::validate::umt_is_value_nan` |
| `isValueNanStr` | `umt_rust::validate::umt_is_value_nan_str` |
| `isValueNanStrLoose` | `umt_rust::validate::umt_is_value_nan_str_loose` |
| `isValueNanStrStrict` | `umt_rust::validate::umt_is_value_nan_str_strict` |
| `validateEmail` | `umt_rust::validate::umt_validate_email` |

## Skipped

| Rust source | Reason |
|---|---|
| `umt_rust::advance::range_advance::umt_range_advance` | generic type parameters |
| `umt_rust::advance::range_advance::umt_range_advance_filtered` | generic type parameters |
| `umt_rust::array::arrays_join::umt_arrays_join` | generic type parameters |
| `umt_rust::array::arrays_join::umt_arrays_join_f64` | unsupported parameter type `& [& [f64]]` |
| `umt_rust::array::arrays_join::umt_arrays_join_two` | generic type parameters |
| `umt_rust::array::binary_search::umt_binary_search` | generic type parameters |
| `umt_rust::array::binary_search::umt_binary_search_i32` | generic type parameters |
| `umt_rust::array::check_flag_alignment::umt_check_flag_alignment` | generic type parameters |
| `umt_rust::array::check_flag_alignment::umt_check_flag_alignment_bool` | unsupported parameter type `& [Vec < bool >]` |
| `umt_rust::array::chunk::umt_chunk` | generic type parameters |
| `umt_rust::array::compact::umt_compact` | generic type parameters |
| `umt_rust::array::compact::umt_compact_options` | generic type parameters |
| `umt_rust::array::compare_function_default::umt_compare_function_default` | generic type parameters |
| `umt_rust::array::drop::umt_drop` | generic type parameters |
| `umt_rust::array::drop::umt_drop_left` | generic type parameters |
| `umt_rust::array::drop::umt_drop_right` | generic type parameters |
| `umt_rust::array::dual_pivot_quick_sort::umt_dual_pivot_quick_sort` | generic type parameters |
| `umt_rust::array::first::umt_first` | generic type parameters |
| `umt_rust::array::get_arrays_common::umt_get_arrays_common` | generic type parameters |
| `umt_rust::array::get_arrays_common::umt_get_arrays_common_f64` | unsupported parameter type `& [& [f64]]` |
| `umt_rust::array::get_arrays_common::umt_get_arrays_common_two` | generic type parameters |
| `umt_rust::array::get_arrays_diff::umt_get_arrays_diff` | generic type parameters |
| `umt_rust::array::get_arrays_diff::umt_get_arrays_diff_two` | generic type parameters |
| `umt_rust::array::group_by::umt_group_by` | generic type parameters |
| `umt_rust::array::group_by::umt_group_by_indexed` | generic type parameters |
| `umt_rust::array::insertion_sort::umt_insertion_sort` | generic type parameters |
| `umt_rust::array::insertion_sort::umt_insertion_sort_in_place` | generic type parameters |
| `umt_rust::array::merge_sort::umt_merge_sort` | generic type parameters |
| `umt_rust::array::pop::umt_pop` | generic type parameters |
| `umt_rust::array::quick_sort::umt_quick_sort` | generic type parameters |
| `umt_rust::array::quick_sort::umt_quick_sort_in_place` | generic type parameters |
| `umt_rust::array::random_select::umt_random_select` | generic type parameters |
| `umt_rust::array::random_select::umt_random_select_one` | generic type parameters |
| `umt_rust::array::shuffle::umt_shuffle` | generic type parameters |
| `umt_rust::array::shuffle::umt_shuffle_in_place` | generic type parameters |
| `umt_rust::array::shuffle_2d_array::umt_shuffle_2d_array` | generic type parameters |
| `umt_rust::array::tim_sort::umt_tim_sort` | generic type parameters |
| `umt_rust::array::tim_sort::umt_tim_sort_in_place` | generic type parameters |
| `umt_rust::array::ultra_number_sort::umt_ultra_number_sort` | unsupported parameter type `& mut Vec < f64 >` |
| `umt_rust::array::uniq_by::umt_uniq_by` | generic type parameters |
| `umt_rust::array::unique::umt_unique` | generic type parameters |
| `umt_rust::array::zip::umt_zip` | generic type parameters |
| `umt_rust::array::zip::umt_zip3` | generic type parameters |
| `umt_rust::array::zip::umt_zip_many` | generic type parameters |
| `umt_rust::array::zip_longest::umt_zip_longest` | generic type parameters |
| `umt_rust::array::zip_longest::umt_zip_longest3` | generic type parameters |
| `umt_rust::array::zip_longest::umt_zip_longest_many` | generic type parameters |
| `umt_rust::async_util::debounce_async::umt_debounce_async` | generic type parameters |
| `umt_rust::async_util::defer::umt_defer` | generic type parameters |
| `umt_rust::async_util::p_settled::umt_p_settled` | generic type parameters |
| `umt_rust::async_util::parallel::umt_parallel` | generic type parameters |
| `umt_rust::async_util::retry::umt_retry` | generic type parameters |
| `umt_rust::async_util::throttle_async::umt_throttle_async` | generic type parameters |
| `umt_rust::async_util::timeout::umt_timeout` | generic type parameters |
| `umt_rust::async_util::wait_for::umt_wait_for` | generic type parameters |
| `umt_rust::color::cmyk_to_rgba::umt_cmyk_to_rgba` | unsupported return type `Rgba` |
| `umt_rust::color::hexa_to_rgba::umt_hexa_to_rgba` | unsupported return type `Result < Rgba , HexColorError >` |
| `umt_rust::color::hsla_to_rgba::umt_hsla_to_rgba` | unsupported return type `Result < Rgba , HslaError >` |
| `umt_rust::color::rgba_to_cmyk::umt_rgba_to_cmyk` | unsupported parameter type `RgbaInput` |
| `umt_rust::color::rgba_to_hexa::umt_rgba_to_hexa` | unsupported parameter type `RgbaInput` |
| `umt_rust::color::rgba_to_hsla::umt_rgba_to_hsla` | unsupported parameter type `RgbaInput` |
| `umt_rust::crypto::decode_base32::umt_decode_base32` | unsupported return type `Result < Vec < u8 > , Base32Error >` |
| `umt_rust::crypto::decode_base32_to_string::umt_decode_base32_to_string` | unsupported return type `Result < String , Base32Error >` |
| `umt_rust::crypto::decode_base58::umt_decode_base58` | unsupported return type `Result < Vec < u8 > , Base58Error >` |
| `umt_rust::crypto::decode_base58_to_string::umt_decode_base58_to_string` | unsupported return type `Result < String , Base58Error >` |
| `umt_rust::crypto::encode_base32::umt_encode_base32` | generic type parameters |
| `umt_rust::crypto::encode_base58::umt_encode_base58` | generic type parameters |
| `umt_rust::date::add_business_days::umt_add_business_days` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::date::add_duration::umt_add_duration` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::date::date_range::umt_date_range` | unsupported parameter type `DateTime < Utc >` |
| `umt_rust::date::date_range::umt_date_range_with_step` | unsupported parameter type `DateTime < Utc >` |
| `umt_rust::date::diff::umt_diff` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::date::end_of::umt_end_of` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::date::format::umt_format` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::date::format::umt_format_iso` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::date::format_relative::umt_format_relative` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::date::from_unix::umt_from_unix` | unsupported parameter type `UnixTimeUnit` |
| `umt_rust::date::get_day::umt_get_day` | unsupported parameter type `DayLanguage` |
| `umt_rust::date::get_day::umt_get_day_en` | unsupported return type `& 'static str` |
| `umt_rust::date::get_day::umt_get_day_ja` | unsupported return type `& 'static str` |
| `umt_rust::date::get_quarter::umt_get_quarter` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::date::is_between::umt_is_between` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::date::is_business_day::umt_is_business_day` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::date::is_same_day::umt_is_same_day` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::date::is_weekend::umt_is_weekend` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::date::ms_by_unit::umt_ms_by_unit` | unsupported parameter type `DurationUnit` |
| `umt_rust::date::new_date::umt_new_date_int` | unsupported return type `Option < DateTime < Utc > >` |
| `umt_rust::date::new_date::umt_new_date_string` | unsupported return type `Option < DateTime < Utc > >` |
| `umt_rust::date::now::umt_now` | unsupported return type `DateTime < Utc >` |
| `umt_rust::date::now::umt_now_jst` | unsupported return type `DateTime < Utc >` |
| `umt_rust::date::start_of::umt_start_of` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::date::sub_business_days::umt_sub_business_days` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::date::sub_duration::umt_sub_duration` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::date::to_unix::umt_to_unix` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::date::week_of_year::umt_week_of_year` | unsupported parameter type `& DateTime < Utc >` |
| `umt_rust::error::error_function::umt_error_function` | generic type parameters |
| `umt_rust::error::flat_map_result::umt_flat_map_result` | generic type parameters |
| `umt_rust::error::map_result::umt_map_result` | generic type parameters |
| `umt_rust::error::match_result::umt_match_result` | generic type parameters |
| `umt_rust::error::retry::umt_retry` | generic type parameters |
| `umt_rust::error::retry::umt_retry_default` | generic type parameters |
| `umt_rust::error::retry::umt_retry_simple` | generic type parameters |
| `umt_rust::error::safe_execute::umt_safe_execute` | generic type parameters |
| `umt_rust::error::safe_execute::umt_safe_execute_mut` | generic type parameters |
| `umt_rust::error::safe_execute::umt_safe_execute_result` | generic type parameters |
| `umt_rust::error::success_function::umt_success_function` | generic type parameters |
| `umt_rust::function::curry::umt_curry0` | generic type parameters |
| `umt_rust::function::curry::umt_curry1` | generic type parameters |
| `umt_rust::function::curry::umt_curry2` | generic type parameters |
| `umt_rust::function::curry::umt_curry3` | generic type parameters |
| `umt_rust::function::curry::umt_curry4` | generic type parameters |
| `umt_rust::function::curry::umt_curry5` | generic type parameters |
| `umt_rust::function::curry::umt_curry6` | generic type parameters |
| `umt_rust::function::debounce::umt_debounce` | generic type parameters |
| `umt_rust::function::memoize::umt_memoize` | generic type parameters |
| `umt_rust::function::memoize::umt_memoize_with_resolver` | generic type parameters |
| `umt_rust::function::once::umt_once` | generic type parameters |
| `umt_rust::function::once::umt_once1` | generic type parameters |
| `umt_rust::function::once::umt_once2` | generic type parameters |
| `umt_rust::function::throttle::umt_throttle` | generic type parameters |
| `umt_rust::map::group_by_to_map::umt_group_by_to_map` | generic type parameters |
| `umt_rust::map::zip_to_map::umt_zip_to_map` | generic type parameters |
| `umt_rust::math::bitwise::umt_bitwise` | unsupported parameter type `RotateDirection` |
| `umt_rust::math::calculator::calculator_initialization::umt_calculator_initialization` | unsupported parameter type `HashMap < String , f64 >` |
| `umt_rust::math::calculator::calculator_initialization::umt_calculator_initialization_fn` | unsupported parameter type `HashMap < String , f64 >` |
| `umt_rust::math::calculator::convert_currency::umt_convert_currency` | unsupported parameter type `Option < & HashMap < String , f64 > >` |
| `umt_rust::math::calculator::convert_currency::umt_convert_currency_string_rates` | unsupported parameter type `Option < & HashMap < String , String > >` |
| `umt_rust::math::calculator::core::umt_calculator_core` | unsupported parameter type `Option < & HashMap < String , f64 > >` |
| `umt_rust::math::calculator::entry::umt_calculator` | unsupported parameter type `Option < & HashMap < String , f64 > >` |
| `umt_rust::math::calculator::entry::umt_calculator_string_rates` | unsupported parameter type `Option < & HashMap < String , String > >` |
| `umt_rust::math::division::umt_division_with_remainder` | unsupported return type `Option < DivisionResult >` |
| `umt_rust::math::linear_congruential_generator::umt_linear_congruential_generator` | generic type parameters |
| `umt_rust::math::prime_factorization::umt_prime_factorization` | unsupported return type `Vec < PrimeFactor >` |
| `umt_rust::math::quotient::umt_quotient` | unsupported return type `(f64 , f64)` |
| `umt_rust::math::reduce::umt_reduce` | unsupported return type `Option < ReduceResult >` |
| `umt_rust::math::repeated_trial::umt_repeated_trial` | unsupported parameter type `Probability` |
| `umt_rust::math::solve_equation::umt_solve_equation` | unsupported parameter type `& mut Vec < Vec < f64 > >` |
| `umt_rust::math::xoshiro256::umt_xoshiro256` | unsupported parameter type `& mut Xoshiro256State` |
| `umt_rust::math::xoshiro256::umt_xoshiro256_01` | unsupported parameter type `& mut Xoshiro256State` |
| `umt_rust::number::format_number::umt_format_number` | unsupported parameter type `& FormatNumberOptions` |
| `umt_rust::object::deep_clone::umt_deep_clone` | unsupported parameter type `& Value` |
| `umt_rust::object::flatten_object::umt_flatten_object` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::get::umt_get` | unsupported parameter type `& Value` |
| `umt_rust::object::get::umt_get_path` | unsupported parameter type `& Value` |
| `umt_rust::object::get_objects_common::umt_get_objects_common` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::get_objects_diff::umt_get_objects_diff` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::has::umt_has` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::has::umt_has_path` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::invert::umt_invert` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::is_empty::umt_is_empty` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::is_plain_object::umt_is_plain_object` | unsupported parameter type `& Value` |
| `umt_rust::object::key_by::umt_key_by` | generic type parameters |
| `umt_rust::object::key_by::umt_key_by_property` | unsupported parameter type `& [Value]` |
| `umt_rust::object::map_keys::umt_map_keys` | generic type parameters |
| `umt_rust::object::map_values::umt_map_values` | generic type parameters |
| `umt_rust::object::merge::umt_merge` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::merge::umt_merge_two` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::merge_deep::umt_merge_deep` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::merge_deep::umt_merge_deep_two` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::omit::umt_omit` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::omit::umt_omit_string_keys` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::omit_by::umt_omit_by` | generic type parameters |
| `umt_rust::object::pick::umt_pick` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::pick::umt_pick_string_keys` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::pick_by::umt_pick_by` | generic type parameters |
| `umt_rust::object::pick_deep::umt_pick_deep` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::pick_deep::umt_pick_deep_string_keys` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::remove_prototype::umt_remove_prototype` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::remove_prototype_deep::umt_remove_prototype_deep` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::object::remove_prototype_map::umt_remove_prototype_map` | unsupported parameter type `& [HashMap < String , Value >]` |
| `umt_rust::object::remove_prototype_map_deep::umt_remove_prototype_map_deep` | unsupported parameter type `& [HashMap < String , Value >]` |
| `umt_rust::object::set::umt_set` | unsupported parameter type `& mut HashMap < String , Value >` |
| `umt_rust::object::set::umt_set_path` | unsupported parameter type `& mut HashMap < String , Value >` |
| `umt_rust::object::unflatten_object::umt_unflatten_object` | unsupported parameter type `& HashMap < String , Value >` |
| `umt_rust::predicate::every::umt_every` | generic type parameters |
| `umt_rust::predicate::is_not_nullish::umt_is_not_nullish` | generic type parameters |
| `umt_rust::predicate::is_nullish::umt_is_nullish` | generic type parameters |
| `umt_rust::predicate::matches::umt_matches` | unsupported parameter type `HashMap < String , Value >` |
| `umt_rust::predicate::not::umt_not` | generic type parameters |
| `umt_rust::predicate::some::umt_some` | generic type parameters |
| `umt_rust::random::random_choice::umt_random_choice` | generic type parameters |
| `umt_rust::random::seeded_random::umt_seeded_random` | unsupported return type `impl FnMut () -> f64` |
| `umt_rust::random::seeded_random::umt_seeded_random_from_str` | unsupported return type `impl FnMut () -> f64` |
| `umt_rust::random::weighted_choice::umt_weighted_choice` | generic type parameters |
| `umt_rust::simple::array::quick_sort_simple::umt_quick_sort_simple` | generic type parameters |
| `umt_rust::simple::date::birthday_simple::umt_birthday_simple_datetime` | unsupported parameter type `& DateTime` |
| `umt_rust::simple::date::birthday_simple::umt_birthday_simple_props` | unsupported parameter type `& BirthdayProperties` |
| `umt_rust::simple::date::day_of_week_simple::umt_day_of_week_simple` | unsupported parameter type `Option < DateProperties >` |
| `umt_rust::simple::date::day_of_week_simple::umt_day_of_week_simple_datetime` | unsupported parameter type `& DateTime` |
| `umt_rust::simple::date::now_simple::umt_new_date` | unsupported return type `DateTime` |
| `umt_rust::simple::date::now_simple::umt_now_simple` | unsupported return type `DateTime` |
| `umt_rust::simple::date::now_simple::umt_now_simple_jst` | unsupported return type `DateTime` |
| `umt_rust::string::format_string::umt_format_string` | unsupported parameter type `& Value` |
| `umt_rust::string::fuzzy_search::umt_fuzzy_search` | unsupported return type `Vec < FuzzyMatch >` |
| `umt_rust::string::fuzzy_search::umt_fuzzy_search_default` | unsupported return type `Vec < FuzzyMatch >` |
| `umt_rust::string::mask::umt_mask` | unsupported parameter type `& MaskOptions` |
| `umt_rust::string::random_string_initialization::umt_random_string_initialization` | unsupported return type `impl Fn (usize) -> String` |
| `umt_rust::string::words::umt_words` | unsupported parameter type `Option < & Regex >` |
| `umt_rust::time::normalize_time_unit::umt_normalize_time_unit` | unsupported parameter type `NormalizeFormat` |
| `umt_rust::tool::create_pipeline::umt_create_pipeline` | generic type parameters |
| `umt_rust::tool::parse_json::umt_parse_json` | generic type parameters |
| `umt_rust::tool::parse_json::umt_parse_json_value` | unsupported return type `Result < serde_json :: Value , JsonParseError >` |
| `umt_rust::tool::pipe::umt_pipe` | generic type parameters |
| `umt_rust::tool::unwrap::umt_unwrap` | generic type parameters |
| `umt_rust::tool::unwrap::umt_unwrap_or` | generic type parameters |
| `umt_rust::tool::unwrap::umt_unwrap_or_else` | generic type parameters |
| `umt_rust::tool::unwrap::umt_unwrap_or_panic` | generic type parameters |
| `umt_rust::ua::extract_browser_from_user_agent::umt_extract_browser_from_user_agent` | unsupported return type `Browser` |
| `umt_rust::ua::extract_device_from_user_agent::umt_extract_device_from_user_agent` | unsupported return type `Device` |
| `umt_rust::ua::extract_os_from_user_agent::umt_extract_os_from_user_agent` | unsupported return type `Os` |
| `umt_rust::ua::parse_user_agent::umt_parse_user_agent` | unsupported return type `UserAgentInfo` |
| `umt_rust::unit::unit_converter::umt_unit_converter` | generic type parameters |
| `umt_rust::url::build_url::umt_build_url` | unsupported parameter type `Option < & HashMap < String , String > >` |
| `umt_rust::url::parse_query_string::umt_parse_query_string` | unsupported return type `HashMap < String , String >` |
| `umt_rust::validate::any::core::umt_any` | generic type parameters |
| `umt_rust::validate::array::umt_array_validator` | generic type parameters |
| `umt_rust::validate::array::umt_validate_array` | generic type parameters |
| `umt_rust::validate::array::array_of::umt_array_of` | generic type parameters |
| `umt_rust::validate::bigint::umt_bigint_validator` | unsupported return type `Box < dyn Fn (i128) -> ValidateCoreReturnType < i128 > >` |
| `umt_rust::validate::bigint::umt_validate_bigint` | unsupported parameter type `i128` |
| `umt_rust::validate::boolean::umt_boolean_validator` | unsupported return type `Box < dyn Fn (bool) -> ValidateCoreReturnType < bool > >` |
| `umt_rust::validate::boolean::umt_validate_boolean` | unsupported return type `ValidateCoreReturnType < bool >` |
| `umt_rust::validate::core::umt_validate_core` | generic type parameters |
| `umt_rust::validate::date::umt_date_validator` | unsupported return type `Box < dyn Fn (Option < DateTime < Utc > >) -> ValidateCoreReturnType < Option < DateTime < Utc > > > >` |
| `umt_rust::validate::date::umt_validate_date` | unsupported parameter type `Option < DateTime < Utc > >` |
| `umt_rust::validate::file::umt_file_validator` | unsupported return type `Box < dyn Fn (Vec < u8 >) -> ValidateCoreReturnType < Vec < u8 > > >` |
| `umt_rust::validate::file::umt_validate_file` | unsupported return type `ValidateCoreReturnType < Vec < u8 > >` |
| `umt_rust::validate::function::umt_func_implement` | generic type parameters |
| `umt_rust::validate::function::umt_validate_func` | generic type parameters |
| `umt_rust::validate::instance_of::umt_instance_of` | generic type parameters |
| `umt_rust::validate::is_array::umt_is_array` | generic type parameters |
| `umt_rust::validate::is_deep_equal::umt_is_deep_equal` | generic type parameters |
| `umt_rust::validate::is_deep_equal::umt_is_deep_equal_map` | generic type parameters |
| `umt_rust::validate::is_deep_equal::umt_is_deep_equal_set` | generic type parameters |
| `umt_rust::validate::is_dictionary_object::umt_is_dictionary_object` | generic type parameters |
| `umt_rust::validate::is_equal::umt_is_equal` | generic type parameters |
| `umt_rust::validate::is_not_empty::umt_is_not_empty` | generic type parameters |
| `umt_rust::validate::is_not_empty::umt_is_not_empty_vec` | generic type parameters |
| `umt_rust::validate::map::umt_validate_map` | generic type parameters |
| `umt_rust::validate::never::umt_never` | generic type parameters |
| `umt_rust::validate::number::umt_double` | unsupported return type `ValidateReturnType < f64 >` |
| `umt_rust::validate::number::umt_even` | unsupported return type `ValidateReturnType < f64 >` |
| `umt_rust::validate::number::umt_max_value` | unsupported return type `ValidateReturnType < f64 >` |
| `umt_rust::validate::number::umt_min_value` | unsupported return type `ValidateReturnType < f64 >` |
| `umt_rust::validate::number::umt_number_validator` | unsupported parameter type `Vec < ValidateReturnType < f64 > >` |
| `umt_rust::validate::number::umt_odd` | unsupported return type `ValidateReturnType < f64 >` |
| `umt_rust::validate::number::umt_prime` | unsupported return type `ValidateReturnType < f64 >` |
| `umt_rust::validate::number::umt_validate_number` | unsupported parameter type `& [ValidateReturnType < f64 >]` |
| `umt_rust::validate::object::umt_optional` | generic type parameters |
| `umt_rust::validate::object::umt_validate_object` | generic type parameters |
| `umt_rust::validate::object::umt_validate_optional` | generic type parameters |
| `umt_rust::validate::object::intersection::umt_intersection` | generic type parameters |
| `umt_rust::validate::object::nullable::umt_nullable` | generic type parameters |
| `umt_rust::validate::object::omit_keys::umt_omit_keys` | generic type parameters |
| `umt_rust::validate::object::partial::umt_partial` | generic type parameters |
| `umt_rust::validate::object::pick_keys::umt_pick_keys` | generic type parameters |
| `umt_rust::validate::object::required::umt_required` | generic type parameters |
| `umt_rust::validate::object::union::umt_union` | generic type parameters |
| `umt_rust::validate::parse_email::umt_parse_email` | unsupported parameter type `Option < ParseEmailOptions >` |
| `umt_rust::validate::set::umt_set_validator` | generic type parameters |
| `umt_rust::validate::set::umt_validate_set` | generic type parameters |
| `umt_rust::validate::string::umt_length` | unsupported return type `ValidateReturnType < String >` |
| `umt_rust::validate::string::umt_max_length` | unsupported return type `ValidateReturnType < String >` |
| `umt_rust::validate::string::umt_min_length` | unsupported return type `ValidateReturnType < String >` |
| `umt_rust::validate::string::umt_number_string` | unsupported return type `ValidateReturnType < String >` |
| `umt_rust::validate::string::umt_regex_match` | unsupported parameter type `Regex` |
| `umt_rust::validate::string::umt_string_validator` | unsupported parameter type `Vec < ValidateReturnType < String > >` |
| `umt_rust::validate::string::umt_uuid` | unsupported parameter type `Option < Vec < u8 > >` |
| `umt_rust::validate::string::umt_validate_email_validator` | unsupported parameter type `Option < ParseEmailLevel >` |
| `umt_rust::validate::string::umt_validate_string` | unsupported parameter type `& [ValidateReturnType < String >]` |
| `umt_rust::validate::string::exact_length::umt_exact_length` | unsupported return type `ValidateReturnType < String >` |
| `umt_rust::validate::string::one_of::umt_one_of` | unsupported return type `impl Fn (& str) -> ValidateCoreReturnType < String >` |
| `umt_rust::validate::template_literal::umt_template_literal` | unsupported parameter type `& [TemplateLiteralPart]` |
| `umt_rust::validate::unknown::core::umt_unknown` | generic type parameters |
