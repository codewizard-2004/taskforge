use taskforge::application::task_service::TaskService;
use taskforge::domain::task::{Priority, Status};

#[test]
fn adds_and_finds_task() {
    let mut service = TaskService::new();

    let id = service.add_task(String::from("Learn Rust"));

    let task = service.get_task(id).unwrap();

    assert_eq!(task.title, "Learn Rust");
    assert_eq!(task.status, Status::Todo);
    assert_eq!(task.priority, Priority::Medium);
}

#[test]
fn changes_task_status() {
    let mut service = TaskService::new();
    let id = service.add_task(String::from("Learn Rust"));

    service.change_status(id, Status::Ongoing);

    assert_eq!(
        service.get_task(id).unwrap().status,
        Status::Ongoing
    );
}

#[test]
fn deleting_missing_task_returns_none() {
    let mut service = TaskService::new();

    assert!(service.delete_task(999).is_none());
}