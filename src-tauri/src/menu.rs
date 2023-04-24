use tauri::{CustomMenuItem, Menu, WindowMenuEvent, Window};

struct View<'a> {
    id: &'a str,
    title: &'a str,
    path: &'a str
}

impl View<'static> {
    fn change_url_path(&self, window: &Window) {
        window.eval(&format!("window.location.replace('http://localhost:1420/{}')", self.path));
    }
}

impl From<View<'static>> for CustomMenuItem {
    fn from(value: View) -> Self {
        CustomMenuItem::new(value.id, value.title)
    }
}

const DB_QUERY_VIEW: View<'static> = View {
    id: "0",
    title: "Query",
    path: "db/query"
};

const DB_TEST_VIEW: View<'static> = View {
    id: "1",
    title: "Test",
    path: "db/test"
};

const VIEWS: [View; 2] = [
    DB_QUERY_VIEW,
    DB_TEST_VIEW
];


pub fn build() -> Menu {
    let mut menu = Menu::new();
    for view in VIEWS {
        menu = menu.add_item(CustomMenuItem::from(view))
    }
    menu
}

pub fn handle_menu_event(event: WindowMenuEvent) {
    VIEWS.iter()
        .find(|view| view.id == event.menu_item_id())
        .unwrap_or(&DB_QUERY_VIEW)
        .change_url_path(event.window())
}
