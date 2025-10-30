mod modules {
    pub mod menu;
    pub mod storage;
    pub mod task;
}

fn main() {
    modules::menu::run_menu();
}
