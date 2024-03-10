use colored::*;
use rusqlite::{Connection, Result};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

struct Task {
    id: i32,
    description: String,
    completed: bool,
}

fn clear_screen() {
    if cfg!(windows) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    } else {
        print!("{esc}c", esc = 27 as char);
    }
    io::stdout().flush().unwrap();
}

fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS \"{}\" (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                completed INTEGER DEFAULT 0
            )",
            table_name
        ),
        [],
    )?;
    Ok(())
}

fn add_task(conn: &Connection, table_name: &str, description: &str) -> Result<()> {
    conn.execute(
        &format!("INSERT INTO \"{}\" (description) VALUES (?)", table_name),
        &[description],
    )?;
    Ok(())
}

fn list_tasks(conn: &Connection, table_name: &str) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare(&format!("SELECT * FROM \"{}\"", table_name))?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task?);
    }

    Ok(tasks)
}

fn mark_task_complete(conn: &Connection, table_name: &str, task_id: i32) -> Result<Vec<Task>> {
    conn.execute(
        &format!("UPDATE \"{}\" SET completed = 1 WHERE id = ?", table_name),
        &[&task_id],
    )?;

    let tasks = list_tasks(&conn, table_name)?;
    Ok(tasks)
}

fn press_enter_to_continue() {
    print!("{}", "Press Enter to continue...".cyan());
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

fn main() -> Result<()> {
    let conn = Connection::open("tasks.db")?;
    create_table(&conn, "default")?;

    loop {
        clear_screen();
        println!("{}", "Todo Application".cyan());
        println!("{}", "1. Add Task".green());
        println!("{}", "2. List Tasks".yellow());
        println!("{}", "3. Mark Task Complete".blue());
        println!("{}", "4. Exit".red());

        let choice: u32 = text_io::read!();

        match choice {
            1 => {
                clear_screen();
                println!("{}", "Enter task description:".cyan());
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                add_task(&conn, "default", &description.trim())?;
                println!("{}", "Task added successfully!".green());
                press_enter_to_continue();
            }
            2 => {
                clear_screen();
                let tasks = list_tasks(&conn, "default")?;
                println!("{}", "Tasks:".yellow());
                for task in tasks {
                    let task_status = if task.completed {
                        "[x]".green()
                    } else {
                        "[ ]".red()
                    };
                    println!("{} {}: {}", task_status, task.id, task.description);
                }
                press_enter_to_continue();
            }
            3 => {
                clear_screen();
                let tasks = list_tasks(&conn, "default")?;
                println!("{}", "Tasks:".yellow());
                for task in tasks {
                    let task_status = if task.completed {
                        "[x]".green()
                    } else {
                        "[ ]".red()
                    };
                    println!("{} {}: {}", task_status, task.id, task.description);
                }

                println!("{}", "Enter task ID to mark as complete:".cyan());
                let task_id: i32 = text_io::read!();
                let tasks = mark_task_complete(&conn, "default", task_id)?;

                println!("{}", "Task marked as complete!".blue());
                println!("{}", "Updated Tasks:".yellow());
                for task in tasks {
                    let task_status = if task.completed {
                        "[x]".green()
                    } else {
                        "[ ]".red()
                    };
                    println!("{} {}: {}", task_status, task.id, task.description);
                }
                press_enter_to_continue();
            }
            4 => {
                clear_screen();
                println!("{}", "Exiting the application.".red());
                thread::sleep(Duration::from_secs(2)); // Add a delay for visibility
                break;
            }
            _ => {
                println!("{}", "Invalid choice. Please choose again.".red());
                press_enter_to_continue();
            }
        }
    }

    Ok(())
}

