use orderbook_rs::prelude::{OrderBook as RustOrderBook};
use pyo3::prelude::*;

#[pyclass]
pub struct PyOrder {
    #[pyo3(get, set)]
    pub id: u64,
    #[pyo3(get, set)]
    pub price: u128,
    #[pyo3(get, set)]
    pub quantity: u64,
    #[pyo3(get, set)]
    pub is_bid: bool, // Simpler for Python than a custom Side enum
}

#[pymethods]
impl PyOrder {
    #[new]
    fn new(id: u64, price: u128, quantity: u64, is_bid: bool) -> Self {
        Self { id, price, quantity, is_bid }; 
    }
}

#[pyclass]
pub struct OrderBook {
    pub order_book: RustOrderBook<()>,
}

#[pymethods]
impl OrderBook {
    #[new]
    pub fn new(symbol: &str) -> Self {
        Self {
            order_book: RustOrderBook::<()>::new(symbol), 
        }
    }

    fn add_order(&mut self, order: OrderType) ->  {
        OrderType::add_order(&mut self.order_book, order);
    }
}

#[pymodule]
fn clob_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    // This name "clob_engine" MUST match the [lib] name in your Cargo.toml
    m.add_class::<OrderBook>()?;
    Ok(())
}
