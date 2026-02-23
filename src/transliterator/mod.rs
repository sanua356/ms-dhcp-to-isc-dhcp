mod belarussian;
mod bulgarian;
mod macedonian;
mod russian;
mod ukrainan;

// Транслитерация реализована по ГОСТ 7.79-2000
pub fn transliterate(source: String) -> String {
    let mut all_alphabets: Vec<(char, &'static str)> = Vec::new();
    all_alphabets.extend_from_slice(belarussian::BELARUSSIAN_TRANSLITERATE_ALPHABET);
    all_alphabets.extend_from_slice(bulgarian::BULGARIAN_TRANSLITERATE_ALPHABET);
    all_alphabets.extend_from_slice(macedonian::MACEDONIAN_TRANSLITERATE_ALPHABET);
    all_alphabets.extend_from_slice(russian::RUSSIAN_TRANSLITERATE_ALPHABET);
    all_alphabets.extend_from_slice(ukrainan::UKRAINAN_TRANSLITERATE_ALPHABET);

    source
        .chars()
        .map(|symbol| {
            let compat = all_alphabets
                .iter()
                .find(|compat_symbol| compat_symbol.0 == symbol);

            match compat {
                Some(item) => item.1.to_string(),
                None => symbol.to_string(),
            }
        })
        .collect::<String>()
}
