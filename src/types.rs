use std::time::SystemTime;

use uuid::Uuid;

pub struct MenuItemId(Uuid);

impl Default for MenuItemId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

pub struct MenuItem {
    pub uid: MenuItemId,
    pub name: String,
    pub cook_time_minutes: u8,
}

impl MenuItem {
    pub fn new(name: String) -> Self {
        Self {
            uid: MenuItemId::default(),
            name,
            cook_time_minutes: get_rand_cooktime(),
        }
    }
}

pub struct OrderId(Uuid);

impl Default for OrderId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

pub struct Order {
    pub uid: OrderId,
    pub table_id: TableId,
    pub menu_item_id: MenuItemId,
}

impl Order {
    fn new(table_id: TableId, menu_item_id: MenuItemId) -> Self {
        Self {
            uid: OrderId::default(),
            table_id,
            menu_item_id,
        }
    }
}

pub struct TableId(Uuid);

impl Default for TableId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

pub struct Table {
    pub uid: TableId,
}

fn get_rand_cooktime() -> u8 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let in_ms = since_the_epoch.as_millis();
    (in_ms % 11) as u8 + 5
}
