// SqlID
#[cfg(any(feature = "mysql"))]
pub type ID = u64;
#[cfg(any(feature = "postgres"))]
pub type ID = i64;

// SqlDateTime
#[cfg(any(feature = "mysql"))]
pub type DateTime = chrono::DateTime<Utc>;
#[cfg(any(feature = "postgres"))]
pub type DateTime = chrono::NaiveDateTime;

// SqlIsDeleted
#[cfg(any(feature = "mysql"))]
pub type U8I16 = u8;
#[cfg(any(feature = "postgres"))]
pub type U8I16 = i16;
