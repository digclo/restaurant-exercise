use std::sync::mpsc::Sender;

use rand::Rng;
use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MenuItemId(Uuid);

impl Default for MenuItemId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone, Copy)]
pub struct MenuItem {
    pub uid: MenuItemId,
    pub name: &'static str,
}

impl MenuItem {
    pub fn new(name: &'static str) -> Self {
        Self {
            uid: MenuItemId::default(),
            name,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OrderId(Uuid);

impl Default for OrderId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Copy, Clone)]
pub struct Order {
    pub uid: OrderId,
    pub table_id: TableId,
    pub menu_item_id: MenuItemId,
    pub cook_time_minutes: u8,
}

impl Order {
    pub fn new(table_id: TableId, menu_item_id: MenuItemId) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            uid: OrderId::default(),
            table_id,
            menu_item_id,
            cook_time_minutes: rng.gen_range(5..15),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TableId(Uuid);

impl Default for TableId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Copy, Clone, Default)]
pub struct Table {
    pub uid: TableId,
}

pub enum Request {
    PostOrder(Sender<Order>, TableId, MenuItemId),
    DeleteOrder(Sender<()>, OrderId),
    GetOrders(Sender<Vec<Order>>, TableId),
    GetOrder(Sender<Option<Order>>, TableId, OrderId),
    GetTables(Sender<Vec<Table>>),
    GetMenuItems(Sender<Vec<MenuItem>>),
}
