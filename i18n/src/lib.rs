mod lang;

use std::ops::Deref;

use crate::lang::ALL_LANGUAGES;

/// get i18n value for ENV `I18N_LANGUAGE` language
///
/// - if ENV `I18N_LANGUAGE` not exist, key will be returned
///
/// # Examples
///
/// ```
/// // a-b-c = "blabla";
///
/// let s = i18n::i18n("a-b-c");
/// println!("{}", s);
/// ```
pub fn i18n(key: &str) -> String {
    let language = std::env::var("I18N_LANGUAGE");
    match language {
        Ok(lang) => i18n_with_language(key, &lang),
        Err(_) => key.to_string(),
    }
}

/// get i18n value for ENV `I18N_LANGUAGE` language with variables
///
/// - if ENV `I18N_LANGUAGE` not exist, key will be returned
///
/// # Examples
///
/// ```
/// // a-b-c = "blabla {0}";
///
/// let s = i18n::i18n_with_vars("a-b-c", vec!["aaa".to_string()]);
/// println!("{}", s);
/// ```
pub fn i18n_with_vars(key: &str, vars: Vec<String>) -> String {
    let language = std::env::var("I18N_LANGUAGE");
    match language {
        Ok(lang) => i18n_with_language_vars(key, &lang, vars),
        Err(_) => key.to_string(),
    }
}

/// get i18n value for param `language`
///
/// - if param `language` not exist, key will be returned
/// - if param `language` exist, key not exist, key will be returned
///
/// # Examples
///
/// ```
/// // a-b-c = "blabla";
///
/// let s = i18n::i18n_with_language("a-b-c", "en");
/// println!("{}", s);
/// ```
pub fn i18n_with_language(key: &str, language: &str) -> String {
    if let Some(v) = ALL_LANGUAGES.get(language) {
        let lang = v.deref().to_owned();
        if let Some(vlang) = lang.get(key) {
            (**vlang).to_string()
        } else {
            key.to_string()
        }
    } else {
        key.to_string()
    }
}

/// get i18n value for param `language` with variables
///
/// - if param `language` not exist, key will be returned
/// - if param `language` exist, key not exist, key will be returned
///
/// # Examples
///
/// ```
/// // a-b-c = "blabla {0}";
///
/// let s = i18n::i18n_with_language_vars("a-b-c", "en", vec!["aaa".to_string()]);
/// println!("{}", s);
/// ```
pub fn i18n_with_language_vars(key: &str, language: &str, vars: Vec<String>) -> String {
    if let Some(v) = ALL_LANGUAGES.get(language) {
        let lang = v.deref().to_owned();
        if let Some(vlang) = lang.get(key) {
            let mut ret = (**vlang).to_string();
            for (i, v) in (0_u8..).zip(vars.into_iter()) {
                let p = vec!["{".to_string(), i.to_string(), "}".to_string()];
                ret = ret.replace(p.join("").as_str(), &v);
            }
            ret
        } else {
            key.to_string()
        }
    } else {
        key.to_string()
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::{i18n, i18n_with_language, i18n_with_language_vars, i18n_with_vars};

    #[test]
    fn test_i18n() {
        std::env::set_var("I18N_LANGUAGE", "en");

        let r = i18n("something-wrong");
        println!("something-wrong = {}", r);
        assert_eq!("Something went wrong.", r);
    }

    #[test]
    fn test_i18n_with_vars() {
        std::env::set_var("I18N_LANGUAGE", "en");

        let r = i18n_with_vars("adventure-not-exist", vec!["2222".to_string()]);
        println!("adventure-not-exist = {}", r);
        assert_eq!("There is no adventure with id 2222.", r);

        std::env::set_var("I18N_LANGUAGE", "zh_CN");

        let r = i18n_with_vars("adventure-not-exist", vec!["2222".to_string()]);
        println!("adventure-not-exist = {}", r);
        assert_eq!("没有这条探险数据 2222", r);
    }

    #[test]
    fn test_i18n_with_language() {
        let r = i18n_with_language("something-wrong", "zh_CN");
        println!("something-wrong = {}", r);
        assert_eq!("发生错误了", r);
    }

    #[test]
    fn test_i18n_with_language_vars() {
        let r = i18n_with_language_vars("adventure-not-exist", "zh_CN", vec!["7788".to_string()]);
        println!("adventure-not-exist = {}", r);
        assert_eq!("没有这条探险数据 7788", r);
    }

    #[test]
    fn test_i18n_with_language_with_wrong_key() {
        let r = i18n_with_language("a-b-c-d", "zh_CN");
        println!("key = {}", r);
        assert_eq!("a-b-c-d", r);
    }
}
