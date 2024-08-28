use std::{
    sync::mpsc::{sync_channel, Sender},
    thread,
};

use tablet::start_tablet;
use types::{MenuItem, MenuItemId, Order, OrderId, Table, TableId};

mod store;
mod tablet;
mod types;

enum Request {
    PostOrder(Sender<Order>, TableId, MenuItemId),
    DeleteOrder(Sender<()>, OrderId),
    GetOrders(Sender<Vec<Order>>, TableId),
    GetOrder(Sender<Option<Order>>, TableId, OrderId),
    GetTables(Sender<Vec<Table>>),
    GetMenuItems(Sender<Vec<MenuItem>>),
}

// Add configuration, and UI to show request count and table activity
fn main() {
    let mut data_store = store::DataStore::default();
    let (tx, rx) = sync_channel::<Request>(1);

    let mut request_count: u128 = 0;

    [0; 10].iter().for_each(|_| {
        let requester = tx.clone();
        thread::spawn(move || {
            start_tablet(requester);
        });
    });

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
        if request_count % 10000 == 0 {
            println!("{}", request_count);
        }
    }
}
