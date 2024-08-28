use crate::types::Request;
use rand::{seq::SliceRandom, Rng};
use std::{sync::mpsc::Sender, thread, time::Duration};

pub fn start_tablet(tx: Sender<Request>, wait_ms: u64) {
    let mut rng = rand::thread_rng();

    let tables = tablet_api::get_tables(tx.clone());
    let menu_items = tablet_api::get_menu_items(tx.clone());

    loop {
        let random_table = tables.choose(&mut rng).unwrap();
        let table_orders = tablet_api::get_orders(tx.clone(), random_table.uid);
        let order_count = table_orders.len();
        match order_count {
            0 => {
                let chosen_item = menu_items.choose(&mut rng).unwrap();
                tablet_api::post_order(tx.clone(), random_table.uid, chosen_item.uid);
            }
            1..9 => {
                let roll = rng.gen_range(1..=3);
                match roll {
                    1 => {
                        let chosen_item = table_orders.choose(&mut rng).unwrap();
                        tablet_api::get_order(tx.clone(), random_table.uid, chosen_item.uid);
                    }
                    2 => {
                        let chosen_order = table_orders.choose(&mut rng).unwrap();
                        tablet_api::delete_order(tx.clone(), chosen_order.uid);
                    }
                    _ => {
                        let chosen_item = menu_items.choose(&mut rng).unwrap();
                        tablet_api::post_order(tx.clone(), random_table.uid, chosen_item.uid);
                    }
                }
            }
            _ => table_orders.iter().for_each(|order| {
                tablet_api::delete_order(tx.clone(), order.uid);
            }),
        };
        if wait_ms != 0 {
            thread::sleep(Duration::from_millis(rng.gen_range(0..wait_ms)));
        }
    }
}

pub mod tablet_api {
    use std::sync::mpsc::{channel, Sender};

    use crate::types::{MenuItem, MenuItemId, Order, OrderId, Request, Table, TableId};

    pub fn get_tables(request: Sender<Request>) -> Vec<Table> {
        let (tx, rx) = channel::<Vec<Table>>();
        request.send(Request::GetTables(tx)).unwrap();
        rx.recv().unwrap()
    }

    pub fn get_menu_items(request: Sender<Request>) -> Vec<MenuItem> {
        let (tx, rx) = channel::<Vec<MenuItem>>();
        request.send(Request::GetMenuItems(tx)).unwrap();
        rx.recv().unwrap()
    }

    pub fn get_orders(request: Sender<Request>, table_id: TableId) -> Vec<Order> {
        let (tx, rx) = channel::<Vec<Order>>();
        request.send(Request::GetOrders(tx, table_id)).unwrap();
        rx.recv().unwrap()
    }

    pub fn get_order(
        request: Sender<Request>,
        table_id: TableId,
        order_id: OrderId,
    ) -> Option<Order> {
        let (tx, rx) = channel::<Option<Order>>();
        request
            .send(Request::GetOrder(tx, table_id, order_id))
            .unwrap();
        rx.recv().unwrap()
    }

    pub fn post_order(
        request: Sender<Request>,
        table_id: TableId,
        menu_item_id: MenuItemId,
    ) -> Order {
        let (tx, rx) = channel::<Order>();
        request
            .send(Request::PostOrder(tx, table_id, menu_item_id))
            .unwrap();
        rx.recv().unwrap()
    }

    pub fn delete_order(request: Sender<Request>, order_id: OrderId) {
        let (tx, rx) = channel::<()>();
        request.send(Request::DeleteOrder(tx, order_id)).unwrap();
        rx.recv().unwrap();
    }
}
