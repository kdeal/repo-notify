use reqwest;

static TODOIST_API: &str = "https://beta.todoist.com/API/v8";

#[derive(Deserialize,Debug)]
pub struct Task {
    pub id: u32,
    pub project_id: u32,
    pub content: String,
    pub completed: bool,
    pub label_ids: Option<Vec<u32>>,
    pub order: u32,
    pub indent: u32,
    pub priority: u32,
    pub due: Option<Due>,
    pub url: String,
    pub comment_count: u32,
}

#[derive(Deserialize,Debug)]
pub struct Due {
    pub string: String,
    pub date: String,
    pub datetime: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug,Default,Serialize)]
pub struct NewTask {
    pub content: String,
    pub project_id: Option<u32>,
    pub order: Option<u32>,
    pub label_ids: Option<Vec<u32>>,
    pub priority: Option<u32>,
    pub due_string: Option<String>,
    pub due_date: Option<String>,
    pub due_datetime: Option<String>,
    pub due_lang: Option<String>,
}

#[derive(Debug,Default,Serialize)]
pub struct NewComment {
    pub task_id: Option<u32>,
    pub project_id: Option<u32>,
    pub content: String,
}

#[derive(Deserialize,Debug)]
pub struct Created {
    pub id: u32,
}



pub fn get_tasks(token: &String) -> Vec<Task> {
    let url = format!("{}/tasks?token={}", TODOIST_API, token);
    let mut resp = reqwest::get(url.as_str()).unwrap();
    assert!(resp.status().is_success());
    resp.json().unwrap()
}

// TODO: Make shared post function
pub fn comment(comment: NewComment, token: &String) -> () {
    let url = format!("{}/comments?token={}", TODOIST_API, token);
    let client = reqwest::Client::new().unwrap();
    let resp = client.post(url.as_str()).unwrap()
        .json(&comment).unwrap()
        .send().unwrap();
    assert!(resp.status().is_success());
}

pub fn create_task(task: NewTask, token: &String) -> u32 {
    let url = format!("{}/tasks?token={}", TODOIST_API, token);
    let client = reqwest::Client::new().unwrap();
    let mut resp = client.post(url.as_str()).unwrap()
        .json(&task).unwrap()
        .send().unwrap();
    assert!(resp.status().is_success());
    let created: Created = resp.json().unwrap();
    created.id
}
