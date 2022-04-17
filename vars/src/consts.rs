use once_cell::sync::Lazy;
use std::collections::HashMap;

const NONE: &str = "";

static ITEM_TYPE: Lazy<HashMap<i16, &str>> = Lazy::new(|| {
    let mut map = HashMap::<i16, &str>::new();
    map.insert(0, "全部");
    map.insert(1, "日常");
    map.insert(2, "搞笑");
    map.insert(3, "游戏");
    map.insert(4, "影视");
    map.insert(5, "旅游");
    map.insert(6, "饭馆");
    map.insert(7, "好物");

    map
});

pub fn to_item_type_name(item_type: i16) -> String {
    match ITEM_TYPE.get(&item_type) {
        Some(v) => v.to_string(),
        None => NONE.into(),
    }
}

static SOURCE_TYPE: Lazy<HashMap<i16, &str>> = Lazy::new(|| {
    let mut map = HashMap::<i16, &str>::new();
    map.insert(1, "抖音");
    map.insert(2, "哔哩哔哩");
    map.insert(3, "西瓜视频");

    map
});

pub fn to_source_name(source: i16) -> String {
    match SOURCE_TYPE.get(&source) {
        Some(v) => v.to_string(),
        None => NONE.to_owned(),
    }
}

static JOURNEY_DESTINY: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::<&str, &str>::new();

    // 省
    map.insert("henan", "河南");
    map.insert("hebei", "河北");
    map.insert("shanxi_jin", "山西");
    map.insert("shandong", "山东");
    map.insert("guangxi", "广西");
    map.insert("guangdong", "广东");
    map.insert("fujian", "福建");
    map.insert("anhui", "安徽");
    map.insert("guizhou", "贵州");
    map.insert("hainan", "海南");
    map.insert("hunan", "湖南");
    map.insert("hubei", "湖北");
    map.insert("jiangxi", "江西");
    map.insert("zhejiang", "浙江");
    map.insert("sichuan", "四川");
    map.insert("xizang", "西藏");
    map.insert("qinghai", "青海");
    map.insert("xinjiang", "新疆");
    map.insert("gansu", "甘肃");
    map.insert("ningxia", "宁夏");
    map.insert("neimenggu", "内蒙古");
    map.insert("shanxi_shan", "陕西");
    map.insert("jiangsu", "江苏");
    map.insert("heilongjiang", "黑龙江");
    map.insert("jilin", "吉林");
    map.insert("liaoning", "辽宁");
    map.insert("yunnan", "云南");
    // 直辖市
    map.insert("chongqing", "重庆");
    map.insert("beijing", "北京");
    map.insert("shanghai", "上海");
    map.insert("tianjin", "天津");
    // 港澳台
    map.insert("xianggang", "香港");
    map.insert("aomen", "澳门");
    map.insert("taiwan", "台湾");

    map
});

pub fn to_journey_destiny_name(code: &str) -> String {
    match JOURNEY_DESTINY.get(&code) {
        Some(v) => v.to_string(),
        None => NONE.to_owned(),
    }
}
