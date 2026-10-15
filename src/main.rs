use std::rc::Rc;

use slint::{ModelRc, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = ToDoList::new()?;
    let ui_weak = ui.as_weak();

    let tasks = Rc::new(VecModel::from(vec![ToDoListItem {
        done: false,
        text: "Walk the dog".into(),
    }]));

    let tasks_rc = ModelRc::from(tasks.clone());
    ui.set_tasks(tasks_rc);

    ui.on_task_inserted(move || {
        let ui = ui_weak.unwrap();
        tasks.push(ToDoListItem {
            text: ui.get_new_task_name(),
            done: false,
        });
        println!("{}", ui.get_new_task_name());
        ui.set_new_task_name("".into());
        println!("__{}__", ui.get_new_task_name());
    });

    ui.run()
}
