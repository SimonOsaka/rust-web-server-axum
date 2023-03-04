use std::collections::HashMap;
use toml::Table;
use once_cell::sync::Lazy;

pub(crate) static ALL_LANGUAGES: Lazy<HashMap<&str, Table>> = Lazy::new(|| {
    let mut all = HashMap::<&str, Table>::new();

    let zh_cn_toml = include_str!("languages/zh_CN.toml");
    let zh_cn_language: Table = toml::from_str(zh_cn_toml).unwrap();
    all.insert("zh_CN", zh_cn_language);

    let en_toml = include_str!("languages/en.toml");
    let en_language: Table = toml::from_str(en_toml).unwrap();
    all.insert("en", en_language);

    all
});