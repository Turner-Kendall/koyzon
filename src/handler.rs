use crate::{
    model::{QueryOptions, Task, UpdateSchema},
    response::{GenericResponse, SingleTaskResponse, TaskData, TaskListResponse},
    WebResult, DB,
};

use chrono::prelude::*;
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, reply::with_status, Reply};


pub async fn ping() -> WebResult<impl Reply> {
    const MESSAGE: &str = "pong";
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(json(response_json))
}

pub async fn test() -> WebResult<impl Reply> {
    const MESSAGE: &str = "This is a test, this is only a test...";
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(json(response_json))
}


pub async fn tasks_list_handler(opts: QueryOptions, db: DB) -> WebResult<impl Reply> {
    
    let tasks = db.lock().await;
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let tasks: Vec<Task> = tasks.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = TaskListResponse {
        status: "success".to_string(),
        results: tasks.len(),
        tasks,
    };

    Ok(json(&json_response))

}


pub async fn create_task_handler(mut body: Task, db: DB) -> WebResult<impl Reply> {
    let mut vec = db.lock().await;

    for task in vec.iter() {
        if task.title == body.title {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Task with title: '{}' already exists", task.title),
            };
            return Ok(with_status(json(&error_response), StatusCode::CONFLICT));
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.created_at = Some(datetime);
    body.updated_at = Some(datetime);

    let task = body.to_owned();

    vec.push(body);

    let json_response = SingleTaskResponse {
        status: "success".to_string(),
        data: TaskData { task },
    };

    Ok(with_status(json(&json_response), StatusCode::CREATED))

}


pub async fn get_task_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let vec = db.lock().await;

    for task in vec.iter() {
        if task.id == Some(id.to_owned()) {
            let json_response = SingleTaskResponse {
                status: "success".to_string(),
                data: TaskData { task: task.clone() },
            };

            return Ok(with_status(json(&json_response), StatusCode::OK));
        }
    }

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Task with ID: {} not found", id),
    };

    return Ok(with_status(json(&error_response), StatusCode::NOT_FOUND));
}


pub async fn edit_task_handler(
    id: String,
    body: UpdateSchema,
    db: DB,
) -> WebResult<impl Reply> {
    let mut vec = db.lock().await;

    for task in vec.iter_mut() {
        if task.id == Some(id.clone()) {
            let datetime = Utc::now();
            let title = body.title.to_owned().unwrap_or(task.title.to_owned());
            let content = body.content.to_owned().unwrap_or(task.content.to_owned());
            let payload = Task {
                id: task.id.to_owned(),
                title: if !title.is_empty() {
                    title
                } else {
                    task.title.to_owned()
                },
                content: if !content.is_empty() {
                    content
                } else {
                    task.content.to_owned()
                },
                completed: if body.completed.is_some() {
                    body.completed
                } else {
                    task.completed
                },
                created_at: task.created_at,
                updated_at: Some(datetime),
            };
            *task = payload;

            let json_response = SingleTaskResponse {
                status: "success".to_string(),
                data: TaskData { task: task.clone() },
            };
            return Ok(with_status(json(&json_response), StatusCode::OK));
        }
    }

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Task with ID: {} not found", id),
    };

    Ok(with_status(json(&error_response), StatusCode::NOT_FOUND))
}


pub async fn delete_task_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let mut vec = db.lock().await;

    for task in vec.iter_mut() {
        if task.id == Some(id.clone()) {
            vec.retain(|task| task.id != Some(id.to_owned()));
            return Ok(with_status(json(&""), StatusCode::NO_CONTENT));
        }
    }

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Task with ID: {} not found", id),
    };

    Ok(with_status(json(&error_response), StatusCode::NOT_FOUND))
}


//

pub async fn file_save_handler(db: DB) -> WebResult<impl Reply> {
    let vec = db.lock().await;

    // for task in vec.iter() {
    //     if task.id == Some(id.to_owned()) {
    //         let json_response = SingleTaskResponse {
    //             status: "success".to_string(),
    //             data: TaskData { task: task.clone() },
    //         };
 
    //         return Ok(with_status(json(&json_response), StatusCode::OK));
    //     }
    // }

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Task with ID:not found"),
    };

    return Ok(with_status(json(&error_response), StatusCode::NOT_FOUND));

}
