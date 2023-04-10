use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};


pub fn build() -> Menu {
    let start = CustomMenuItem::new("start".to_string(), "Start");
    let stop = CustomMenuItem::new("stop".to_string(), "Stop");
    let scraper_menu = Submenu::new("Scraper", Menu::new().add_item(start).add_item(stop));
    Menu::new().add_submenu(scraper_menu)
}

pub fn handle_menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "start" => {
            println!("Scrapper->Start invoked");
        }
        "stop" => {
            println!("Scrapper->Stop invoked");
        }
        _ => {}
    }
}