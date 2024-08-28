use std::sync::mpsc::{Sender, channel};
use rand::{Rng, seq::SliceRandom};

use crate::{Request, types::{Table, Order, TableId, OrderId, MenuItemId, MenuItem}};

pub fn start_tablet(tx: Sender<Request>) {
    let mut rng = rand::thread_rng();

    let tables = get_tables(&tx);
    let menu_items = get_menu_items(&tx);

    loop {
        let random_table = tables.choose(&mut rng).unwrap();
        let table_orders = get_orders(&tx, random_table.uid);
        let no_orders = table_orders.is_empty();
        match no_orders {
            true =>{
                let chosen_item = menu_items.choose(&mut rng).unwrap();
                post_order(&tx,random_table.uid, chosen_item.uid);
            },
            false => {
                let roll = rng.gen_range(1..=4);
                match roll {
                    1 => {
                        let chosen_item = table_orders.choose(&mut rng).unwrap();
                        get_order(&tx,random_table.uid, chosen_item.uid).unwrap();
                    }, 
                    2 =>{
                        let chosen_order = table_orders.choose(&mut rng).unwrap();
                        delete_order(&tx, chosen_order.uid);
                    },
                    _ => {
                        let chosen_item = menu_items.choose(&mut rng).unwrap();
                        post_order(&tx,random_table.uid, chosen_item.uid);
                    }, 
                }
            }
        };
    }
}

fn get_tables(request:&Sender<Request>) -> Vec<Table> {
    let (tx,rx) = channel::<Vec<Table>>();
    request.send(Request::GetTables(tx)).unwrap();
    let response = rx.recv().unwrap();
    println!("Successfully got tables");
    response
}

fn get_menu_items(request: &Sender<Request>) -> Vec<MenuItem> {
    let (tx,rx) = channel::<Vec<MenuItem>>();
    request.send(Request::GetMenuItems(tx)).unwrap();
    let response = rx.recv().unwrap();
    println!("Successfully got menu items");
    response
}

fn get_orders(request:&Sender<Request>, table_id: TableId) -> Vec<Order> {
    let (tx,rx) = channel::<Vec<Order>>();
    request.send(Request::GetOrders(tx, table_id)).unwrap();
    let response = rx.recv().unwrap();
    println!("Successfully got orders");
    response
}

fn get_order(request:&Sender<Request>, table_id: TableId, order_id: OrderId) -> Option<Order> {
    let (tx,rx) = channel::<Option<Order>>();
    request.send(Request::GetOrder(tx, table_id, order_id)).unwrap();
    let response = rx.recv().unwrap();
    println!("Successfully got single order");
    response
}

fn post_order(request:&Sender<Request>, table_id: TableId, menu_item_id:MenuItemId) -> Order {
    let (tx,rx) = channel::<Order>();
    request.send(Request::PostOrder(tx, table_id, menu_item_id)).unwrap();
    let response = rx.recv().unwrap();
    println!("Successfully created single order");
    response
}

fn delete_order(request:&Sender<Request>, order_id: OrderId) {
    let (tx,rx) = channel::<()>();
    request.send(Request::DeleteOrder(tx, order_id)).unwrap();
    rx.recv().unwrap();
    println!("Successfully deleted single order");
}
