pub const FORMAT_PATTERN_FULL: &str =
    "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6] \
                    [offset_hour sign:mandatory]:[offset_minute]";

pub const FORMAT_PATTERN_YMD_HMS: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";
pub const FORMAT_PATTERN_YMD_HMS_TZ: &str =
    "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]";

pub const FORMAT_PATTERN_YMD: &str = "[year]-[month]-[day]";
pub const FORMAT_PATTERN_HMS: &str = "[hour]:[minute]:[second]";

pub const FORMAT_PATTERN_YEAR: &str = "[year]";
pub const FORMAT_PATTERN_MONTH: &str = "[month]";
pub const FORMAT_PATTERN_DAY: &str = "[day]";

pub const FORMAT_PATTERN_HOUR: &str = "[hour]";
pub const FORMAT_PATTERN_MINUTE: &str = "[minute]";
pub const FORMAT_PATTERN_SECOND: &str = "[second]";
