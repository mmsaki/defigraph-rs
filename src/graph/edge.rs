use rust_decimal::Decimal;

use super::pool::Pool;

#[derive(Debug, PartialEq, Eq)]
pub struct Edge<'a> {
    pub pool: &'a Pool<'a>,
    pub weight: Decimal,
}

impl<'a> Edge<'a> {
    pub fn new(pool: &'a Pool, weight: Decimal) -> Self {
        Self { pool, weight }
    }
}
