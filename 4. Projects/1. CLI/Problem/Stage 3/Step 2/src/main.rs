use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    // TODO: create database and navigator

    let db = Rc::new(JiraDatabase::new("./data/db.json".to_owned()));
    let mut navigator = Navigator::new(Rc::clone(&db));
    
    loop {
        clearscreen::clear().unwrap();

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.

        let current_page = navigator.get_current_page();

        match current_page {
            Some(cp) => {
                if let Err(e) = cp.draw_page() {
                    println!(
                        "Error rendering page: {}\nPress any key to continue...",
                        e
                    );
                    wait_for_key_press();
                }

                let user_input = get_user_input();

                match cp.handle_input(user_input.trim()) {
                    Err(error) => {
                        println!(
                            "Error getting user input: {}\nPress any key to continue...",
                            error
                        );
                        wait_for_key_press();
                    }
                    Ok(action) => {
                        if let Some(action) = action {
                            if let Err(error) = navigator.handle_action(action) {
                                println!("Error handling processing user input: {}\nPress any key to continue...", error);
                                wait_for_key_press();
                            }
                        }
                    }
                }
            },
            None => break
        }

        // 2. render page
        // 3. get user input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action
    }
}