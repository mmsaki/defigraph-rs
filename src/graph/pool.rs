use rust_decimal::Decimal;

use crate::graph::vertex::Vertex;

#[derive(Debug, PartialEq, Eq)]
pub struct Pool<'a> {
    pub pool_address: String,
    pub token0: &'a Vertex,
    pub token1: &'a Vertex,
    pub fee: usize,
    pub token0_price: Decimal,
    pub token1_price: Decimal,
}

impl<'a> Pool<'a> {
    pub fn new(
        pool_address: String,
        token0: &'a Vertex,
        token1: &'a Vertex,
        fee: usize,
        token0_price: Decimal,
        token1_price: Decimal,
    ) -> Self {
        Self {
            pool_address,
            token0,
            token1,
            fee,
            token0_price,
            token1_price,
        }
    }
}
