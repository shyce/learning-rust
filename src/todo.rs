use std::io::{self, Write};

#[derive(Debug)]
struct Todo {
    pub id: u16,
    pub task: String
}

pub fn main() {
  let mut todos: Vec<Todo> = Vec::new();
  let mut next_id: u16 = 1; 

  loop {
      let choice: String = crate::helper::create_menu("Todo App", &vec![
        "Create Todo",
        "Read Todos",
        "Update Todo",
        "Delete Todo",
      ]);

      match choice.as_str() {
          "1" => {
              let new_todo = create_todo(next_id);
              todos.push(new_todo);
              next_id += 1;
              read_todos(&todos);
          },
          "2" => read_todos(&todos),
          "3" => update_todos(&mut todos),
          "4" => delete_todos(&mut todos),
          "exit" => break,
          _ => println!("Invalid choice, please try again."),
      }
  }
}

fn create_todo(next_id: u16) -> Todo {
  print!("Task: ");
  io::stdout().flush().expect("Could not flush stdout");
  let user_input: String = crate::helper::read();
  Todo{ 
      id: next_id, 
      task: user_input.trim().to_string(),
  }
}

fn read_todos(todos: &Vec<Todo>) {
  if todos.len() == 0 {
    return;
  } 
  println!("\n~~~~~~~~~~~~~~~");
  for todo in todos {
      println!("{:?}", todo);
  }
  println!("~~~~~~~~~~~~~~~");
}

fn update_todos(todos: &mut Vec<Todo>) {
  println!("Which task id are we updating?");
  println!("---------------");
  read_todos(&todos);
  println!("---------------");
  let id_to_update = crate::helper::read();
  if let Some(todo) = todos.iter_mut().find(|t| t.id == id_to_update) {
      println!("Current task: {}", todo.task);
      println!("Enter the new task:");
      todo.task = crate::helper::read(); 
  } else {
      println!("No task found with ID {}", id_to_update);
  }
}

fn delete_todos(todos: &mut Vec<Todo>) {
  println!("Which task id are we deleting?");
  println!("---------------");
  read_todos(&todos);
  println!("---------------");
  let id_to_delete = crate::helper::read();
  if let Some(index) = todos.iter_mut().position(|t| t.id == id_to_delete) {
      todos.remove(index);
  } else {
      println!("No task found with ID {}", id_to_delete);
  }
}