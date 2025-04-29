use std::collections::HashMap;

pub enum Status {
    InProgress,
    Closed,
    Open
}

pub struct Epic {
    name: String,
    description: String,
    status: Status,
    stories: Vec<i32>
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open and the stories should be an empty vector
        Epic {
            name: name,
            description: description,
            status: Status::Open,
            stories: Vec::new()
        }
    }
}

pub struct Story {
    name: String,
    description: String,
    status: Status
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open
        Story {
            name: name,
            description: description,
            status: Status::Open,
        }
    }
}

pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // TODO: add fields (make sure the fields are public)
    last_item_id: i32,
    epics: Vec<Epic>,
    stories: Vec<Story>
}