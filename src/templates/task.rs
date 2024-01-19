use crate::templates::staff::AssignedStaff;
use askama::Template;
use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::types::uuid;
use uuid::Uuid;

use crate::{
    models::{AcceptanceStatus, EventRole, TaskPriority},
    repositories::{
        event_staff::models::StaffLite, task::models::TaskExtended, user::models::UserLite,
    },
};

use super::user::UserLiteTemplate;

// #[derive(Template, Deserialize)]
// #[template(path = "event/task/task.html")]
// pub struct TaskTemplate {
//     pub id: Uuid,
//     pub event_id: Uuid,
//     pub creator_id: Uuid, // staff table ID
//     pub creator: UserLiteTemplate,
//     pub title: String,
//     pub description: String,
//     pub finished_at: Option<NaiveDateTime>,
//     pub priority: TaskPriority,
//     pub accepts_staff: bool,
//     pub created_at: NaiveDateTime,
//     pub edited_at: NaiveDateTime,
// }

// impl From<TaskExtended> for TaskTemplate {
//     fn from(task: TaskExtended) -> Self {
//         let creator_lite: UserLite = task.creator.into();
//         let creator = creator_lite.into();

//         TaskTemplate {
//             id: task.task_id,
//             event_id: task.event_id,
//             creator_id: task.creator_id,
//             creator,
//             title: task.title,
//             description: if task.description.is_some() {
//                 task.description.unwrap()
//             } else {
//                 "No description.".to_string()
//             },
//             finished_at: task.finished_at,
//             priority: task.priority,
//             accepts_staff: task.accepts_staff,
//             created_at: task.created_at,
//             edited_at: task.edited_at,
//         }
//     }
// }

#[derive(Deserialize, Debug)]
pub struct EventTask {
    pub id: Uuid,
    pub event_id: Uuid,
    pub creator_id: Uuid, // staff table ID
    pub creator: UserLiteTemplate,
    pub title: String,
    pub description: String,
    pub finished_at: Option<NaiveDateTime>,
    pub priority: TaskPriority,
    pub accepts_staff: bool,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

impl From<TaskExtended> for EventTask {
    fn from(task: TaskExtended) -> Self {
        let creator_lite: UserLite = task.creator.into();
        let creator = creator_lite.into();

        EventTask {
            id: task.task_id,
            event_id: task.event_id,
            creator_id: task.creator_id,
            creator,
            title: task.title,
            description: if task.description.is_some() {
                task.description.unwrap()
            } else {
                "No description.".to_string()
            },
            finished_at: task.finished_at,
            priority: task.priority,
            accepts_staff: task.accepts_staff,
            created_at: task.created_at,
            edited_at: task.edited_at,
        }
    }
}

#[derive(Template, Deserialize)]
#[template(path = "event/task/tasks.html")]
pub struct TasksTemplate {
    pub tasks: Vec<EventTask>,
}

#[derive(Template, Deserialize)]
#[template(path = "event/task/tasks-panel.html")]
pub struct TasksPanelTemplate {
    pub requester: StaffLite,
}

#[derive(Template, Deserialize)]
#[template(path = "event/task/task-panel.html")]
pub struct TaskPanelTemplate {
    pub requester_id: Uuid,
    pub assigned_staff: Option<AssignedStaff>,
    pub task: EventTask,
}

#[derive(Template, Deserialize)]
#[template(path = "event/task/task-creation.html")]
pub struct TaskCreationTemplate {
    pub creator_id: Uuid,
    pub event_id: Uuid,
}

#[derive(Template, Deserialize)]
#[template(path = "event/task/task-edit.html")]
pub struct TaskEditTemplate {
    pub editor_id: Uuid,
    pub task: EventTask,
}
