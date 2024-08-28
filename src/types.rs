use rand::Rng;
use uuid::Uuid;

#[derive(Copy, Clone, PartialEq)]
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
    pub cook_time_minutes: u8,
}

impl MenuItem {
    pub fn new(name: &'static str) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            uid: MenuItemId::default(),
            name,
            cook_time_minutes: rng.gen_range(5..15),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
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
}

impl Order {
    pub fn new(table_id: TableId, menu_item_id: MenuItemId) -> Self {
        Self {
            uid: OrderId::default(),
            table_id,
            menu_item_id,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
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
