use leptos::*;
use unic_langid_impl::LanguageIdentifier;

pub fn get_language(input: &str) -> String {
    LanguageIdentifier::from_bytes(input.as_bytes())
        .unwrap()
        .to_string()
}

fn main() {
    let lang_str = get_language("en-Latn-US-variant3-variant3-variant2");
    mount_to_body(|| view! { <p>{lang_str}</p> })
}
