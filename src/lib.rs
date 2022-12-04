use tidb_query_datatype::codec::table;

// Range represents a backup response.
#[derive(Default, Debug)]
pub struct Range {
    pub start_key: Vec<u8>,
    pub end_key: Vec<u8>,
}

impl Range {
    pub fn new(table_id: i64) -> Self {
        let start_key = table::encode_row_key(table_id.to_owned(), i64::MIN);
        let end_key = table::encode_row_key(table_id, i64::MAX);
        Range { start_key, end_key }
    }
}
