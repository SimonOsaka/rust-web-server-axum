#[derive(Clone, Debug)]
pub struct AdventuresFilter {
    pub item_id: u8,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub province_key: Option<String>,
}

#[derive(Clone, Debug)]
pub struct PlayListFilter {
    pub play_list: String,
}
