use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
pub struct Database {
    conn: Connection,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("todo.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT 0
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn add_task(&self, description: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO tasks (description, completed) VALUES (?1, 0)",
            params![description],
        )?;
        Ok(())
    }

    pub fn list_tasks(&self) -> Result<Vec<(i32, String, bool)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, description, completed FROM tasks")?;
        let tasks = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
            .collect::<Result<Vec<_>>>()?;
        Ok(tasks)
    }

    pub fn complete_task(&self, id: i32) -> Result<()> {
        self.conn
            .execute("UPDATE tasks SET completed = 1 WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn remove_task(&self, id: i32) -> Result<()> {
        self.conn
            .execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
        Ok(())
    }
    pub fn export_to_json(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let tasks = self
            .list_tasks()?
            .into_iter()
            .map(|(id, description, completed)| Task {
                id,
                description,
                completed,
            })
            .collect::<Vec<Task>>();

        let json = serde_json::to_string_pretty(&tasks)?;
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }
    pub fn import_from_json(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(_) => {
                eprintln!("❌ Error: Could'nt read the file '{}'. Verify if your file exists and ensure the file have the correct access rights.", filename);
                return Ok(());
            }
        };

        let tasks: Vec<Value> = match from_str(&json) {
            Ok(data) => data,
            Err(_) => {
                eprintln!("❌ Error: Incorrect JSON format.\n\n📌 Expected format:\n[\n  {{\"id\": 1, \"description\": \"Buy milk\", \"completed\": false}},\n  {{\"id\": 2, \"description\": \"Workout\", \"completed\": true}}\n]\n\n🔎 Ensure each one of the tasks must have the following properties: 'id' (number), 'description' (string) y 'completed' (bool).");
                return Ok(());
            }
        };

        for task in &tasks {
            let id = task
                .get("id")
                .and_then(Value::as_i64)
                .unwrap_or_else(|| self.get_next_id());

            let description = task
                .get("description")
                .and_then(Value::as_str)
                .unwrap_or("Task without description");

            let completed = task
                .get("completed")
                .and_then(Value::as_bool)
                .unwrap_or(false);

            self.conn.execute(
                "INSERT INTO tasks (id, description, completed) VALUES (?1, ?2, ?3)
                 ON CONFLICT(id) DO UPDATE SET description = excluded.description, completed = excluded.completed",
                params![id, description, completed],
            )?;
        }

        println!(
            "✅ Import process succeed: Added {} taks from '{}'",
            tasks.len(),
            filename
        );
        Ok(())
    }
    pub fn import_from_json_dry_run(
        &self,
        filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let tasks: Vec<Task> = serde_json::from_reader(reader)?;

        println!("Tasks to be imported:");

        for task in tasks {
            let status = if task.completed { "✔️" } else { "❌" };
            println!("Description: {} - Status: {}", task.description, status);
        }

        Ok(())
    }
    fn get_next_id(&self) -> i64 {
        let mut stmt = self.conn.prepare("SELECT MAX(id) FROM tasks").unwrap();
        let max_id: Option<i64> = stmt.query_row([], |row| row.get(0)).unwrap();
        max_id.unwrap_or(0) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{Connection, Result};
    use std::fs::File;
    use std::io::Write;

    struct TestDatabase {
        db: Database,
    }

    impl TestDatabase {
        fn new() -> Self {
            let conn = Connection::open_in_memory().unwrap();
            conn.execute(
                "CREATE TABLE tasks (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    description TEXT NOT NULL,
                    completed BOOLEAN NOT NULL DEFAULT 0
                )",
                [],
            )
            .unwrap();
            TestDatabase {
                db: Database { conn },
            }
        }
    }

    #[test]
    fn test_add_task() {
        let test_db = TestDatabase::new();
        assert!(test_db.db.add_task("Test task").is_ok());
    }

    #[test]
    fn test_list_tasks() {
        let test_db = TestDatabase::new();
        test_db.db.add_task("Task 1").unwrap();
        test_db.db.add_task("Task 2").unwrap();
        let tasks = test_db.db.list_tasks().unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].1, "Task 1");
        assert_eq!(tasks[1].1, "Task 2");
    }

    #[test]
    fn test_complete_task() {
        let test_db = TestDatabase::new();
        test_db.db.add_task("Incomplete Task").unwrap();
        let tasks = test_db.db.list_tasks().unwrap();
        let task_id = tasks[0].0;
        test_db.db.complete_task(task_id).unwrap();
        let updated_tasks = test_db.db.list_tasks().unwrap();
        assert!(updated_tasks.iter().any(|t| t.0 == task_id && t.2));
    }

    #[test]
    fn test_remove_task() {
        let test_db = TestDatabase::new();
        test_db.db.add_task("Task to Remove").unwrap();
        let tasks = test_db.db.list_tasks().unwrap();
        let task_id = tasks[0].0;
        test_db.db.remove_task(task_id).unwrap();
        let tasks_after = test_db.db.list_tasks().unwrap();
        assert!(tasks_after.is_empty());
    }

    #[test]
    fn test_export_to_json() {
        let test_db = TestDatabase::new();
        test_db.db.add_task("Export Task").unwrap();
        let filename = "test_export.json";
        assert!(test_db.db.export_to_json(filename).is_ok());
        assert!(std::fs::metadata(filename).is_ok());
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_import_from_json() {
        let test_db = TestDatabase::new();
        let filename = "test_import.json";
        let mut file = File::create(filename).unwrap();
        writeln!(
            file,
            "[{{\"id\": 1, \"description\": \"Imported Task\", \"completed\": false}}]"
        )
        .unwrap();
        assert!(test_db.db.import_from_json(filename).is_ok());
        let tasks = test_db.db.list_tasks().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].1, "Imported Task");
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_import_from_json_dry_run() {
        let test_db = TestDatabase::new();
        let filename = "test_dry_run.json";
        let mut file = File::create(filename).unwrap();
        writeln!(
            file,
            "[{{\"id\": 1, \"description\": \"Dry Run Task\", \"completed\": true}}]"
        )
        .unwrap();
        assert!(test_db.db.import_from_json_dry_run(filename).is_ok());
        std::fs::remove_file(filename).unwrap();
    }
}
