use sqlx::types::chrono::NaiveDateTime;

#[inline]
pub fn naive_date_format(date: NaiveDateTime) -> String {
    format!("{}", date.format("%Y-%m-%d %H:%M:%S"))
}
