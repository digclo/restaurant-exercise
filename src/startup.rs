use std::sync::mpsc::Receiver;

use crate::{
    store::DataStore,
    types::{Order, Request},
};

pub fn spawn_app(rx: Receiver<Request>, table_count: usize, print_interval: u64) {
    let mut data_store = DataStore::new(table_count);
    let mut request_count: u64 = 0;
    while let Ok(request) = rx.recv() {
        use Request as R;
        match request {
            R::PostOrder(response, table_id, menu_item_id) => {
                let new_order = Order::new(table_id, menu_item_id);
                data_store.insert_order(&new_order).unwrap();
                response.send(new_order).unwrap();
            }
            R::DeleteOrder(response, order_id) => {
                data_store.delete_order(&order_id);
                response.send(()).unwrap();
            }
            R::GetOrders(response, table_id) => {
                let orders = data_store.get_orders_by_table(&table_id);
                response.send(orders).unwrap();
            }
            R::GetOrder(response, table_id, order_id) => {
                let order = data_store.get_order_by_uid(&table_id, &order_id);
                response.send(order).unwrap();
            }
            R::GetTables(response) => {
                let tables = data_store.get_tables();
                response.send(tables).unwrap();
            }
            R::GetMenuItems(response) => {
                let menu_items = data_store.get_menu_items();
                response.send(menu_items).unwrap();
            }
        }
        request_count += 1;
        if print_interval != 0 && request_count % print_interval == 0 {
            println!("{}", request_count);
        }
    }
}
