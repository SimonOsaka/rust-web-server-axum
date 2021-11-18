use crate::db::SqlArguments;
use sqlx::Arguments;
use tracing::debug;
pub struct SqlParams {
    index: u8,
    placeholder: String,
    args: SqlArguments,
}

impl SqlParams {
    pub fn new() -> SqlParams {
        SqlParams {
            index: 1,
            placeholder: String::from(""),
            args: SqlArguments::default(),
        }
    }

    pub fn add_value<'q, T>(&mut self, value: T) -> String
    where
        T: 'q
            + Send
            + sqlx::Encode<'q, sqlx::Postgres>
            + sqlx::Type<sqlx::Postgres>
            + std::fmt::Debug,
    {
        self.placeholder = format!("${:?}", self.index);
        self.index = self.index + 1;
        debug!("add_value: {} = {:?}", self.placeholder, value);
        self.args.add(value);
        self.placeholder.clone()
    }

    pub fn fetch(self) -> SqlArguments {
        self.args
    }
}

#[cfg(test)]
mod tests {

    use sql_builder::SqlBuilder;
    use tracing::debug;

    use crate::db::params::SqlParams;

    #[test]
    fn sql_delete_param() {
        let mut sql_param = SqlParams::new();
        let mut sql_builder = SqlBuilder::delete_from("table");

        let sql = sql_builder
            .and_where_eq("id", sql_param.add_value(123))
            .sql()
            .unwrap();
        debug!("{:?}", &sql);
        assert_eq!(sql, "DELETE FROM table WHERE id = $1;");
    }
}
