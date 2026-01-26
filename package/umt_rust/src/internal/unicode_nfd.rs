/// Performs a simplified NFD (Canonical Decomposition) on a string,
/// removing combining diacritical marks (U+0300..U+036F).
///
/// This handles common Latin accented characters by decomposing them
/// to their base character + combining mark, then filtering out the marks.
pub fn strip_diacritics(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        if let Some(base) = decompose_char(c) {
            result.push(base);
        } else if !is_combining_mark(c) {
            result.push(c);
        }
    }
    result
}

fn is_combining_mark(c: char) -> bool {
    ('\u{0300}'..='\u{036F}').contains(&c)
}

/// Returns the base character for common accented characters, or None if
/// the character doesn't need decomposition.
fn decompose_char(c: char) -> Option<char> {
    match c {
        // Latin capital letters with diacritics
        'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' => Some('A'),
        'Ç' => Some('C'),
        'È' | 'É' | 'Ê' | 'Ë' => Some('E'),
        'Ì' | 'Í' | 'Î' | 'Ï' => Some('I'),
        'Ð' => Some('D'),
        'Ñ' => Some('N'),
        'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' => Some('O'),
        'Ù' | 'Ú' | 'Û' | 'Ü' => Some('U'),
        'Ý' => Some('Y'),
        // Latin small letters with diacritics
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => Some('a'),
        'ç' => Some('c'),
        'è' | 'é' | 'ê' | 'ë' => Some('e'),
        'ì' | 'í' | 'î' | 'ï' => Some('i'),
        'ð' => Some('d'),
        'ñ' => Some('n'),
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' => Some('o'),
        'ù' | 'ú' | 'û' | 'ü' => Some('u'),
        'ý' | 'ÿ' => Some('y'),
        // Extended Latin
        'Ā' | 'Ă' | 'Ą' => Some('A'),
        'ā' | 'ă' | 'ą' => Some('a'),
        'Ć' | 'Ĉ' | 'Ċ' | 'Č' => Some('C'),
        'ć' | 'ĉ' | 'ċ' | 'č' => Some('c'),
        'Ď' | 'Đ' => Some('D'),
        'ď' | 'đ' => Some('d'),
        'Ē' | 'Ĕ' | 'Ė' | 'Ę' | 'Ě' => Some('E'),
        'ē' | 'ĕ' | 'ė' | 'ę' | 'ě' => Some('e'),
        'Ĝ' | 'Ğ' | 'Ġ' | 'Ģ' => Some('G'),
        'ĝ' | 'ğ' | 'ġ' | 'ģ' => Some('g'),
        'Ĥ' | 'Ħ' => Some('H'),
        'ĥ' | 'ħ' => Some('h'),
        'Ĩ' | 'Ī' | 'Ĭ' | 'Į' | 'İ' => Some('I'),
        'ĩ' | 'ī' | 'ĭ' | 'į' | 'ı' => Some('i'),
        'Ĵ' => Some('J'),
        'ĵ' => Some('j'),
        'Ķ' => Some('K'),
        'ķ' => Some('k'),
        'Ĺ' | 'Ļ' | 'Ľ' | 'Ŀ' | 'Ł' => Some('L'),
        'ĺ' | 'ļ' | 'ľ' | 'ŀ' | 'ł' => Some('l'),
        'Ń' | 'Ņ' | 'Ň' => Some('N'),
        'ń' | 'ņ' | 'ň' => Some('n'),
        'Ō' | 'Ŏ' | 'Ő' => Some('O'),
        'ō' | 'ŏ' | 'ő' => Some('o'),
        'Ŕ' | 'Ŗ' | 'Ř' => Some('R'),
        'ŕ' | 'ŗ' | 'ř' => Some('r'),
        'Ś' | 'Ŝ' | 'Ş' | 'Š' => Some('S'),
        'ś' | 'ŝ' | 'ş' | 'š' => Some('s'),
        'Ţ' | 'Ť' | 'Ŧ' => Some('T'),
        'ţ' | 'ť' | 'ŧ' => Some('t'),
        'Ũ' | 'Ū' | 'Ŭ' | 'Ů' | 'Ű' | 'Ų' => Some('U'),
        'ũ' | 'ū' | 'ŭ' | 'ů' | 'ű' | 'ų' => Some('u'),
        'Ŵ' => Some('W'),
        'ŵ' => Some('w'),
        'Ŷ' | 'Ÿ' => Some('Y'),
        'ŷ' => Some('y'),
        'Ź' | 'Ż' | 'Ž' => Some('Z'),
        'ź' | 'ż' | 'ž' => Some('z'),
        // Not a decomposable character
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_basic() {
        assert_eq!(strip_diacritics("café"), "cafe");
        assert_eq!(strip_diacritics("naïve"), "naive");
        assert_eq!(strip_diacritics("hello"), "hello");
        assert_eq!(strip_diacritics(""), "");
    }

    #[test]
    fn test_strip_various() {
        assert_eq!(strip_diacritics("Ã"), "A");
        assert_eq!(strip_diacritics("ñ"), "n");
        assert_eq!(strip_diacritics("ü"), "u");
    }
}
