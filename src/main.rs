pub mod graph;
use graph::edge::Edge;
use graph::graph::Graph;
use graph::pool::Pool;
use graph::vertex::Vertex;
use rust_decimal::{Decimal, MathematicalOps};
use std::collections::HashMap;

pub fn run() -> String {
    let v1 = Vertex::new(String::from("ETH"), 18, String::from("0x0001"));
    let v2 = Vertex::new(String::from("USDC"), 18, String::from("0x0002"));
    let pool_address = "0x2323";

    let token0: &Vertex = &v1;
    let token1: &Vertex = &v2;
    let fee = 500;
    let token0_price = Decimal::new(3034, 25);
    let token1_price = Decimal::new(1034, 28);

    let pool = Pool::new(
        pool_address.to_string(),
        token0,
        token1,
        fee,
        token0_price,
        token1_price,
    );

    let weight = MathematicalOps::log10(&token0_price);

    let edge = Edge::new(&pool, weight);

    let ref_v1: &Vertex = &v1;
    let ref_v2: &Vertex = &v2;

    let e1 = &edge;
    let e2 = &edge;
    let mut adjascency_list: HashMap<&Vertex, Vec<&Edge>> = HashMap::new();
    adjascency_list.entry(ref_v1).or_insert(Vec::new()).push(e1);
    adjascency_list.entry(ref_v2).or_insert(Vec::new()).push(e2);

    let g = Graph::new(adjascency_list);
    format!("{:#?}", g)
}

fn main() {
    println!("{}", run())
}
