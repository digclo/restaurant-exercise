use std::convert::Infallible;

use crate::types::{MenuItem, Order, OrderId, Table, TableId};

const MENU: [&str; 5] = ["Ramen", "Soba", "Udon", "Tendon", "Katsudon"];

pub struct DataStore {
    orders: Vec<Order>,
    menu_items: Vec<MenuItem>,
    tables: Vec<Table>,
}

impl DataStore {
    pub fn new(table_count: usize) -> Self {
        let menu_items = MENU.iter().map(|name| MenuItem::new(name)).collect();
        let tables: Vec<Table> = (0..table_count).map(|_| Table::default()).collect();
        Self {
            orders: Vec::default(),
            menu_items,
            tables,
        }
    }
}

impl DataStore {
    pub fn insert_order(&mut self, order: &Order) -> Result<(), Infallible> {
        self.orders.push(*order);
        Ok(())
    }

    pub fn delete_order(&mut self, target_id: &OrderId) -> Option<Order> {
        if let Some(pos) = self.orders.iter().position(|order| order.uid == *target_id) {
            Some(self.orders.swap_remove(pos))
        } else {
            None
        }
    }

    pub fn get_orders_by_table(&mut self, target_id: &TableId) -> Vec<Order> {
        self.orders
            .iter()
            .filter(|&order| order.table_id == *target_id)
            .cloned()
            .collect()
    }

    pub fn get_order_by_uid(&mut self, table_id: &TableId, order_id: &OrderId) -> Option<Order> {
        self.orders
            .iter()
            .filter(|&order| order.table_id == *table_id)
            .find(|&order| order.uid == *order_id)
            .cloned()
    }

    pub fn get_menu_items(&mut self) -> Vec<MenuItem> {
        self.menu_items.to_vec()
    }

    pub fn get_tables(&self) -> Vec<Table> {
        self.tables.to_vec()
    }
}
