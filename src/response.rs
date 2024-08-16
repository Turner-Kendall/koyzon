use crate::model::Task;
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct TaskData {
    pub task: Task,
}

#[derive(Serialize, Debug)]
pub struct SingleTaskResponse {
    pub status: String,
    pub data: TaskData,
}

#[derive(Serialize, Debug)]
pub struct TaskListResponse {
    pub status: String,
    pub results: usize,
    pub tasks: Vec<Task>,
}
