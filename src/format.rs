use toml_edit::{decorated, Array, ArrayOfTables, InlineTable, Item, Table, Value};

/// Format.
pub trait Format {
    fn format(&mut self);
}

impl Format for Item {
    fn format(&mut self) {
        *self = match self {
            Item::None => return,
            Item::Value(value) => {
                value.format();
                Item::Value(value.clone())
            }
            Item::Table(table) => {
                table.format();
                Item::Table(table.clone())
            }
            Item::ArrayOfTables(input_array_of_tables) => {
                let mut output_array_of_tables = ArrayOfTables::new();
                for table in input_array_of_tables.iter() {
                    let mut table = table.clone();
                    table.format();
                    output_array_of_tables.append(table);
                }
                Item::ArrayOfTables(output_array_of_tables)
            }
        }
    }
}

impl Format for Value {
    fn format(&mut self) {
        let value = match &*self {
            Value::Array(input_array) => {
                let mut output_array = Array::default();
                for value in input_array.iter() {
                    let mut value = value.clone();
                    value.format();
                    output_array.push(value);
                }
                output_array.fmt();
                output_array.into()
            }
            Value::InlineTable(input_inline_table) => {
                let mut output_inline_table = InlineTable::default();
                for (key, value) in input_inline_table.iter() {
                    let mut value = value.clone();
                    value.format();
                    output_inline_table.get_or_insert(key, value);
                }
                output_inline_table.fmt();
                output_inline_table.into()
            }
            value => value.clone(),
        };
        *self = decorated(value.into(), " ", "");
    }
}

impl Format for Table {
    fn format(&mut self) {
        let mut table = Table::new();
        for (key, item) in self.iter() {
            let mut item = item.clone();
            item.format();
            *table.entry(key) = item;
        }
        *self = table
    }
}
