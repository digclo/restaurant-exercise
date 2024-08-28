use restaurant::{
    startup::spawn_app,
    tablet::tablet_api,
    types::{MenuItem, Order, Request, Table},
};
use std::{
    sync::mpsc::{channel, Sender},
    thread,
};

#[test]
fn tablet_post_request_should_store_order() {
    let (tx, rx) = channel::<Request>();

    thread::spawn(move || {
        spawn_app(rx,1,0);
    });

    let table = get_a_table(tx.clone());
    let menu_item = get_a_menu_item(tx.clone());

    tablet_api::post_order(tx.clone(), table.uid, menu_item.uid);

    let table_orders = tablet_api::get_orders(tx.clone(), table.uid);
    let new_order = table_orders.first().unwrap();

    assert_eq!(table_orders.len(), 1);
    assert_eq!(new_order.table_id, table.uid);
    assert_eq!(new_order.menu_item_id, menu_item.uid);
    assert!(new_order.cook_time_minutes >= 5 && new_order.cook_time_minutes <= 15);
}

#[test]
fn tablet_delete_request_should_remove_order() {
    let (tx, rx) = channel::<Request>();

    thread::spawn(move || {
        spawn_app(rx,1,0);
    });

    let table = get_a_table(tx.clone());
    let items = tablet_api::get_menu_items(tx.clone());

    let orders: Vec<Order> = items
        .iter()
        .map(|item| tablet_api::post_order(tx.clone(), table.uid, item.uid))
        .collect();
    let target_order = orders.first().unwrap();
    tablet_api::delete_order(tx.clone(), target_order.uid);

    let table_orders = tablet_api::get_orders(tx.clone(), table.uid);
    table_orders.iter().for_each(|order| {
        assert_ne!(order.uid, target_order.uid);
    });
}

#[test]
fn tablet_get_table_orders_request_should_return_all_table_orders() {
    let (tx, rx) = channel::<Request>();

    thread::spawn(move || {
        spawn_app(rx,1,1);
    });

    let table = get_a_table(tx.clone());
    let items = tablet_api::get_menu_items(tx.clone());

    items.iter().for_each(|item| {
        tablet_api::post_order(tx.clone(), table.uid, item.uid);
    });

    let table_orders = tablet_api::get_orders(tx.clone(), table.uid);

    assert_eq!(table_orders.len(), items.len());
    table_orders.iter().for_each(|order| {
        assert_eq!(order.table_id, table.uid);
    });
}

#[test]
fn tablet_get_table_order_request_should_return_target_table_orders() {
    let (tx, rx) = channel::<Request>();

    thread::spawn(move || {
        spawn_app(rx,1,0);
    });

    let table = get_a_table(tx.clone());
    let menu_item = get_a_menu_item(tx.clone());

    let created_order = tablet_api::post_order(tx.clone(), table.uid, menu_item.uid);

    let target_order = tablet_api::get_order(tx.clone(), table.uid, created_order.uid).unwrap();

    assert_eq!(target_order.uid, created_order.uid);
    assert_eq!(target_order.menu_item_id, menu_item.uid);
    assert_eq!(target_order.table_id, table.uid);
}

#[test]
fn tablet_api_should_handle_simultaneous_requests() {
    const THREAD_LIMIT: usize = 10;
    let (tx, rx) = channel::<Request>();
    let (assert_tx, assert_rx) = channel::<()>();

    thread::spawn(move || {
        spawn_app(rx,1,0);
    });

    let table = get_a_table(tx.clone());
    let items = tablet_api::get_menu_items(tx.clone());
    let items_count = items.len();

    [0; THREAD_LIMIT].iter().for_each(|_| {
        let sender = tx.clone();
        let assert_sender = assert_tx.clone();
        let items_clone = items.clone();

        thread::spawn(move || {
            items_clone.iter().for_each(|item| {
                tablet_api::post_order(sender.clone(), table.uid, item.uid);
            });
            assert_sender.send(()).unwrap();
        });
    });

    for _ in 0..THREAD_LIMIT {
        assert_rx.recv().unwrap();
    }

    let resulting_orders = tablet_api::get_orders(tx.clone(), table.uid);
    let expected_count = THREAD_LIMIT * items_count;
    assert_eq!(expected_count, resulting_orders.len());
}

fn get_a_table(tx: Sender<Request>) -> Table {
    let tables = tablet_api::get_tables(tx);
    *tables.first().unwrap()
}

fn get_a_menu_item(tx: Sender<Request>) -> MenuItem {
    let items = tablet_api::get_menu_items(tx);
    *items.first().unwrap()
}
