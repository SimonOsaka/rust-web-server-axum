type Timezone = (i8, u8);

#[cfg(any(feature = "Shanghai"))]
pub const TIMEZONE: Timezone = (8, 0);

#[cfg(any(feature = "Hawaii"))]
pub const TIMEZONE: Timezone = (-10, 0);
