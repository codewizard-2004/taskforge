use taskforge::application::task_service::TaskService;
use taskforge::domain::task::Status;

fn main() {
    println!("TaskForge CLI To-Do App");

    let mut service = TaskService::new();

    service.add_task(String::from("Learn rust basics"));
    service.add_task(String::from("TL;DR the S8 electives"));

    println!("The tasks were created");
    let tasks = service.tasks();
    for task in tasks{
        println!("{}\t{:?}\t{:?}", task.title, task.priority, task.status);
    }

    match service.get_task(5){
        Some(task) => println!("The task was found: {}", task.title),
        None => println!("The task was not found")
    }

    service.change_status(1, Status::Ongoing);

    if let Some(task) = service.get_task(1){
        println!("Task {} Updated", 1);
        println!("{}\t{}\t{:?}", task.id, task.title, task.status);
    }

}
