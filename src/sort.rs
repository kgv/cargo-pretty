use crate::order::{Order, Ordered};
use std::cmp::Ordering;
use toml_lalrpop::value::{Array, Item, Table, Value};

fn cmp(order: &Order, a: &str, b: &str) -> Ordering {
    match order {
        Order::Unordered => Ordering::Equal,
        Order::Ordered(Ordered::Alphabetic) => a.cmp(b),
        Order::Ordered(Ordered::Enumeration(map)) => match (map.get_full(a), map.get_full(b)) {
            (None, None) => a.cmp(b),
            (Some(a), Some(ref b)) => a.cmp(b),
            (a, ref b) => a.cmp(b).reverse(),
        },
    }
}

/// Sort.
pub trait Sort {
    fn sort(&mut self, order: &Order);
}

impl<T: Sort> Sort for &mut T {
    fn sort(&mut self, order: &Order) {
        (*self).sort(order);
    }
}

impl<T: Sort> Sort for Option<T> {
    fn sort(&mut self, order: &Order) {
        if let Some(t) = self.as_mut() {
            t.sort(order);
        }
    }
}

impl Sort for Item {
    fn sort(&mut self, order: &Order) {
        self.value.sort(order);
    }
}

impl Sort for Value {
    fn sort(&mut self, order: &Order) {
        match self {
            Value::Primitive(_) => {}
            Value::Array(array) => array.sort(order),
            Value::Table(table) => table.sort(order),
        }
    }
}

// Place primitive values before array or table. Sort relevant primitive values.
// Other values leave unaltered.
impl Sort for Array {
    fn sort(&mut self, order: &Order) {
        self.sort_by(|a, b| match (&a.value, &b.value) {
            (Value::Primitive(a), Value::Primitive(b)) => {
                cmp(order, &a.to_string(), &b.to_string())
            }
            (Value::Primitive(_), Value::Array(_)) | (Value::Primitive(_), Value::Table(_)) => {
                Ordering::Greater
            }
            (Value::Array(_), Value::Primitive(_)) | (Value::Table(_), Value::Primitive(_)) => {
                Ordering::Less
            }
            _ => Ordering::Equal,
        });
    }
}

impl Sort for Table {
    fn sort(&mut self, order: &Order) {
        self.sort_by(|a, _, b, _| cmp(order, a, b));
    }
}
