pub mod my_item_type_format {
    use crate::types::U8I16;

    const ALL: &str = "全部";

    const DAILY: &str = "日常";
    const HAHA: &str = "搞笑";
    const GAME: &str = "游戏";
    const FILM_TV: &str = "影视";
    const JOURNEY: &str = "旅游";
    const RESTAURANT: &str = "饭馆";
    const RETAIL: &str = "好物";

    const NONE: &str = "";

    pub fn to_item_type_name(item_type: U8I16) -> String {
        match item_type {
            0 => ALL.to_owned(),
            1 => DAILY.to_owned(),
            2 => HAHA.to_owned(),
            3 => GAME.to_owned(),
            4 => FILM_TV.to_owned(),
            5 => JOURNEY.to_owned(),
            6 => RESTAURANT.to_owned(),
            7 => RETAIL.to_owned(),
            _ => NONE.to_owned(),
        }
    }
}

pub mod my_source {
    use crate::types::U8I16;

    const DOUYIN: &str = "抖音";
    const BILIBILI: &str = "哔哩哔哩";
    const XIGUASHIPIN: &str = "西瓜视频";
    const NONE: &str = "";

    pub fn to_source_name(source: U8I16) -> String {
        match source {
            1 => DOUYIN.to_owned(),
            2 => BILIBILI.to_owned(),
            3 => XIGUASHIPIN.to_owned(),
            _ => NONE.to_owned(),
        }
    }
}

pub mod my_journey_destiny {
    // 省
    const HENAN: &str = "河南";
    const HEBEI: &str = "河北";
    const SHANXI_JIN: &str = "山西";
    const SHANDONG: &str = "山东";
    const GUANGXI: &str = "广西";
    const GUANGDONG: &str = "广东";
    const FUJIAN: &str = "福建";
    const ANHUI: &str = "安徽";
    const GUIZHOU: &str = "贵州";
    const HAINAN: &str = "海南";
    const HUNAN: &str = "湖南";
    const HUBEI: &str = "湖北";
    const JIANGXI: &str = "江西";
    const ZHEJIANG: &str = "浙江";
    const SICHUAN: &str = "四川";
    const XIZANG: &str = "西藏";
    const QINGHAI: &str = "青海";
    const XINJIANG: &str = "新疆";
    const GANSU: &str = "甘肃";
    const NINGXIA: &str = "宁夏";
    const NEIMENGGU: &str = "内蒙古";
    const SHANXI_SHAN: &str = "陕西";
    const JIANGSU: &str = "江苏";
    const HEILONGJIANG: &str = "黑龙江";
    const JILIN: &str = "吉林";
    const LIAONING: &str = "辽宁";
    const YUNNAN: &str = "云南";
    // 直辖市
    const CHONGQING: &str = "重庆";
    const BEIJING: &str = "北京";
    const SHANGHAI: &str = "上海";
    const TIANJIN: &str = "天津";
    // 港澳台
    const XIANGGANG: &str = "香港";
    const AOMEN: &str = "澳门";
    const TAIWAN: &str = "台湾";

    const NONE: &str = "";

    pub fn to_name(code: &str) -> String {
        match code {
            "henan" => HENAN.to_owned(),
            "hebei" => HEBEI.to_owned(),
            "shanxi_jin" => SHANXI_JIN.to_owned(),
            "shandong" => SHANDONG.to_owned(),
            "guangxi" => GUANGXI.to_owned(),
            "guangdong" => GUANGDONG.to_owned(),
            "fujian" => FUJIAN.to_owned(),
            "anhui" => ANHUI.to_owned(),
            "guizhou" => GUIZHOU.to_owned(),
            "hainan" => HAINAN.to_owned(),
            "hunan" => HUNAN.to_owned(),
            "hubei" => HUBEI.to_owned(),
            "jiangxi" => JIANGXI.to_owned(),
            "zhejiang" => ZHEJIANG.to_owned(),
            "sichuan" => SICHUAN.to_owned(),
            "qinghai" => QINGHAI.to_owned(),
            "xizang" => XIZANG.to_owned(),
            "xinjiang" => XINJIANG.to_owned(),
            "gansu" => GANSU.to_owned(),
            "ningxia" => NINGXIA.to_owned(),
            "neimenggu" => NEIMENGGU.to_owned(),
            "shanxi_shan" => SHANXI_SHAN.to_owned(),
            "jiangsu" => JIANGSU.to_owned(),
            "heilongjiang" => HEILONGJIANG.to_owned(),
            "jilin" => JILIN.to_owned(),
            "liaoning" => LIAONING.to_owned(),
            "yunnan" => YUNNAN.to_owned(),

            "chongqing" => CHONGQING.to_owned(),
            "beijing" => BEIJING.to_owned(),
            "shanghai" => SHANGHAI.to_owned(),
            "tianjin" => TIANJIN.to_owned(),

            "xianggang" => XIANGGANG.to_owned(),
            "aomen" => AOMEN.to_owned(),
            "taiwan" => TAIWAN.to_owned(),
            _ => NONE.to_owned(),
        }
    }
}
