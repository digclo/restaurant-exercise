use rand::{seq::SliceRandom, Rng};
use std::sync::mpsc::{channel, SyncSender};

use crate::{
    types::{MenuItem, MenuItemId, Order, OrderId, Table, TableId},
    Request,
};

pub fn start_tablet(tx: SyncSender<Request>) {
    let mut rng = rand::thread_rng();

    let tables = get_tables(tx.clone());
    let menu_items = get_menu_items(tx.clone());

    loop {
        let random_table = tables.choose(&mut rng).unwrap();
        let table_orders = get_orders(tx.clone(), random_table.uid);
        let order_count = table_orders.len();
        match order_count {
            0 => {
                let chosen_item = menu_items.choose(&mut rng).unwrap();
                post_order(tx.clone(), random_table.uid, chosen_item.uid);
            }
            1..9 => {
                let roll = rng.gen_range(1..=3);
                match roll {
                    1 => {
                        let chosen_item = table_orders.choose(&mut rng).unwrap();
                        get_order(tx.clone(), random_table.uid, chosen_item.uid);
                    }
                    2 => {
                        let chosen_order = table_orders.choose(&mut rng).unwrap();
                        delete_order(tx.clone(), chosen_order.uid);
                    }
                    _ => {
                        let chosen_item = menu_items.choose(&mut rng).unwrap();
                        post_order(tx.clone(), random_table.uid, chosen_item.uid);
                    }
                }
            }
            _ => {
                let chosen_order = table_orders.choose(&mut rng).unwrap();
                delete_order(tx.clone(), chosen_order.uid);
            }
        };
    }
}

fn get_tables(request: SyncSender<Request>) -> Vec<Table> {
    let (tx, rx) = channel::<Vec<Table>>();
    request.send(Request::GetTables(tx)).unwrap();
    rx.recv().unwrap()
}

fn get_menu_items(request: SyncSender<Request>) -> Vec<MenuItem> {
    let (tx, rx) = channel::<Vec<MenuItem>>();
    request.send(Request::GetMenuItems(tx)).unwrap();
    rx.recv().unwrap()
}

fn get_orders(request: SyncSender<Request>, table_id: TableId) -> Vec<Order> {
    let (tx, rx) = channel::<Vec<Order>>();
    request.send(Request::GetOrders(tx, table_id)).unwrap();
    rx.recv().unwrap()
}

fn get_order(request: SyncSender<Request>, table_id: TableId, order_id: OrderId) -> Option<Order> {
    let (tx, rx) = channel::<Option<Order>>();
    request
        .send(Request::GetOrder(tx, table_id, order_id))
        .unwrap();
    rx.recv().unwrap()
}

fn post_order(request: SyncSender<Request>, table_id: TableId, menu_item_id: MenuItemId) -> Order {
    let (tx, rx) = channel::<Order>();
    request
        .send(Request::PostOrder(tx, table_id, menu_item_id))
        .unwrap();
    rx.recv().unwrap()
}

fn delete_order(request: SyncSender<Request>, order_id: OrderId) {
    let (tx, rx) = channel::<()>();
    request.send(Request::DeleteOrder(tx, order_id)).unwrap();
    rx.recv().unwrap();
}
