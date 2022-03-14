pub mod error;
pub mod format;
pub mod timezone;

use time::{format_description, OffsetDateTime, UtcOffset};

use self::{
    error::DateError,
    format::{
        FORMAT_PATTERN_DAY, FORMAT_PATTERN_HOUR, FORMAT_PATTERN_MINUTE, FORMAT_PATTERN_MONTH,
        FORMAT_PATTERN_SECOND, FORMAT_PATTERN_YEAR, FORMAT_PATTERN_YMD_HMS_TZ,
    },
};

pub type DateResult = Result<OffsetDateTime, DateError>;
pub type DateStringResult = Result<String, DateError>;

/// `OffsetDateTime`'s util
/// - Format `OffsetDateTime` to String
/// - Parse String to `OffsetDateTime`
#[derive(Debug, Clone)]
pub struct Date;

impl Date {
    /// get utc `OffsetDateTime`
    #[cfg(not(feature = "local"))]
    pub fn get_now() -> DateResult {
        Ok(OffsetDateTime::now_utc())
    }

    /// get local `OffsetDateTime`
    #[cfg(any(feature = "local"))]
    pub fn get_now() -> DateResult {
        use self::timezone::TIMEZONE;

        Self::get_now_with_tz(TIMEZONE.0, TIMEZONE.1)
    }

    /// now unix timestamp
    pub fn get_now_unix_timestamp() -> Result<i64, DateError> {
        Ok(Self::get_now()?.unix_timestamp())
    }

    /// from `timestamp` to `OffsetDateTime`
    pub fn from_unix_timestamp(ts: i64) -> DateResult {
        OffsetDateTime::from_unix_timestamp(ts).map_err(|e| DateError::FormatRange { e })
    }

    /// get datetime with timezone
    pub fn get_now_with_tz(hour: i8, minute: u8) -> DateResult {
        Ok(OffsetDateTime::now_utc().to_offset(
            UtcOffset::from_hms(hour, minute as i8, 0).map_err(|e| DateError::FormatRange { e })?,
        ))
    }

    /// format any datetime and pattern
    ///
    /// ```
    /// [year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]
    /// ```
    pub fn format_with(odt: OffsetDateTime, fmt: &str) -> DateStringResult {
        let f = format_description::parse(fmt)
            .map_err(|e| DateError::FormatInvalid { fmt: fmt.into(), e })?;
        odt.format(&f).map_err(|e| DateError::Format { e })
    }

    /// format year
    ///
    /// ```
    /// [year]
    /// ```
    pub fn format_with_year(odt: OffsetDateTime) -> DateStringResult {
        Self::format_with(odt, FORMAT_PATTERN_YEAR)
    }

    /// format month
    ///
    /// ```
    /// [month]
    /// ```
    pub fn format_with_month(odt: OffsetDateTime) -> DateStringResult {
        Self::format_with(odt, FORMAT_PATTERN_MONTH)
    }

    /// format day
    ///
    /// ```
    /// [day]
    /// ```
    pub fn format_with_day(odt: OffsetDateTime) -> DateStringResult {
        Self::format_with(odt, FORMAT_PATTERN_DAY)
    }

    /// format year:month:day
    ///
    /// ```
    /// [year]-[month]-[day]
    /// ```
    pub fn format_with_ymd(odt: OffsetDateTime) -> DateStringResult {
        Ok(format!(
            "{}-{}-{}",
            Self::format_with_year(odt)?,
            Self::format_with_month(odt)?,
            Self::format_with_day(odt)?
        ))
    }

    /// format hour
    ///
    /// ```
    /// [hour]
    /// ```
    pub fn format_with_hour(odt: OffsetDateTime) -> DateStringResult {
        Self::format_with(odt, FORMAT_PATTERN_HOUR)
    }

    /// format minute
    ///
    /// ```
    /// [minute]
    /// ```
    pub fn format_with_minute(odt: OffsetDateTime) -> DateStringResult {
        Self::format_with(odt, FORMAT_PATTERN_MINUTE)
    }

    /// format second
    ///
    /// ```
    /// [second]
    /// ```
    pub fn format_with_second(odt: OffsetDateTime) -> DateStringResult {
        Self::format_with(odt, FORMAT_PATTERN_SECOND)
    }

    /// format hour:minute:second
    ///
    /// ```
    /// [hour]:[minute]:[second]
    /// ```
    pub fn format_with_hms(odt: OffsetDateTime) -> DateStringResult {
        Ok(format!(
            "{}:{}:{}",
            Self::format_with_hour(odt)?,
            Self::format_with_minute(odt)?,
            Self::format_with_second(odt)?
        ))
    }

    /// format year-month-day hour:minute:second
    ///
    /// ```
    /// '[year]-[month]-[day] [hour]:[minute]:[second]'
    /// ```
    pub fn format_with_ymd_hms(odt: OffsetDateTime) -> DateStringResult {
        Ok(format!(
            "{} {}",
            Self::format_with_ymd(odt)?,
            Self::format_with_hms(odt)?
        ))
    }

    /// parse date string with given pattern to OffsetDateTime
    ///
    /// ```
    /// "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]"
    /// "2022-07-01 02:30:57 +00:30"
    /// ```
    pub fn parse_with(date_str: &str, pattern: &str) -> DateResult {
        let format = format_description::parse(pattern).map_err(|e| {
            DateError::ParseInvalidPatternError {
                pattern: pattern.into(),
                e,
            }
        })?;

        OffsetDateTime::parse(date_str, &format).map_err(|e| DateError::ParseError {
            pattern: pattern.into(),
            e,
        })
    }

    /// parse date and time string to OffsetDateTime
    ///
    /// ```
    /// Pattern: [year]-[month]-[day] [hour]:[minute]:[second]
    /// Example: "2022-07-01 02:30:57"
    /// ```
    pub fn parse_with_ymd_hms(date_str: &str) -> DateResult {
        Self::parse_with(
            format!("{} {}", date_str, Self::gen_timezone()).as_str(),
            FORMAT_PATTERN_YMD_HMS_TZ,
        )
    }

    /// parse date string to OffsetDateTime
    ///
    /// ```
    /// Pattern: [year]-[month]-[day]
    /// Example: "2022-07-01"
    /// ```
    pub fn parse_with_ymd(date_str: &str) -> DateResult {
        Self::parse_with(
            format!("{} 00:00:00 {}", date_str, Self::gen_timezone()).as_str(),
            FORMAT_PATTERN_YMD_HMS_TZ,
        )
    }

    #[cfg(not(feature = "local"))]
    fn gen_timezone() -> String {
        String::from("+00:00")
    }

    #[cfg(any(feature = "local"))]
    fn gen_timezone() -> String {
        use self::timezone::TIMEZONE;

        let mut sign = "+";
        let mut offset_hour = TIMEZONE.0;
        if TIMEZONE.0 < 0 {
            sign = "-";
            offset_hour = -offset_hour;
        }
        format!("{sign}{0:0>2}:{1:0>2}", offset_hour, TIMEZONE.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::Date;
    #[test]
    fn test_today() {
        let now = Date::get_now();
        println!("{now:?}");
        let odt = now.unwrap();
        println!("{odt}");
        let s_date = Date::format_with_ymd(odt);
        println!("{s_date:?}");
        let s_time = Date::format_with_hms(odt);
        println!("{s_time:?}");
        let tz_date = Date::get_now_with_tz(25, 0);
        println!("{tz_date:?}");
        let s_ymd_hms = Date::format_with_ymd_hms(odt);
        println!("{s_ymd_hms:?}");
    }

    #[test]
    fn test_parse_with() {
        let s_ymd_hms = Date::parse_with_ymd_hms("2022-05-11 18:15:12");
        println!("{s_ymd_hms:?}");

        let s_ymd_hms = Date::parse_with_ymd("2022-05-12");
        println!("{s_ymd_hms:?}");

        let s_ymd_hms = Date::parse_with(
            "2022-05-11 00:00:00 +08:00",
            "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]",
        );
        println!("{s_ymd_hms:?}");
    }

    #[test]
    fn test_gen_timezone() {
        let tz = Date::gen_timezone();
        println!("{tz}");
    }
}
