use std::convert::Infallible;

use crate::types::{Order, OrderId, TableId, MenuItem, MenuItemId, Table};


#[derive(Default)]
pub struct DataStore {
    orders: Vec<Order>,
    menu_items: Vec<MenuItem>,
    tables: Vec<Table>,
}

impl DataStore {
    pub fn insert_order(&mut self, order: &Order) -> Result<(), Infallible>  {
        self.orders.push(*order);
        Ok(())
    }

    pub fn delete_order(&mut self, target_id: &OrderId) -> Option<Order>{
        if let Some(pos) = self.orders.iter().position(|order|order.uid == *target_id) {
            Some(self.orders.swap_remove(pos))
        } else {
            None
        }
    }

    pub fn get_orders_by_table(&mut self, target_id: &TableId) -> Vec<Order> {
        self.orders.iter()
            .filter(|&order|order.table_id == *target_id)
            .cloned().collect()
    }

    pub fn get_order_by_uid(&mut self, target_id: &OrderId) -> Option<Order> {
        self.orders.iter()
            .find(|&order|order.uid == *target_id)
            .cloned()
    }

    pub fn insert_menu_item(&mut self, item: &MenuItem) -> Result<(), Infallible>  {
        self.menu_items.push(*item);
        Ok(())
    }


    pub fn get_menu_item_by_uid(&mut self, target_id: &MenuItemId) -> Option<MenuItem> {
        self.menu_items.iter()
            .find(|&item|item.uid == *target_id)
            .cloned()
    }
    
    pub fn insert_table(&mut self, table: &Table) -> Result<(), Infallible>  {
        self.tables.push(*table);
        Ok(())
    }

    pub fn get_tables(&mut self) -> Vec<Table> {
        self.tables.to_vec()
    }

}

