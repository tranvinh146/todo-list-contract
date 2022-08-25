// /*
//  * Example smart contract written in RUST
//  *
//  * L&earn more about &writing NEAR smart contracts with Rust:
//  * https://near-docs.io/develop/Contract
//  *
//  */
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::json_types::U64;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId};

// Define the contract structure
#[near_bindgen]
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct TodoList {
    pub accounts: UnorderedMap<AccountId, Tasks>,
}

#[near_bindgen]
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Tasks {
    pub in_progress: Vector<String>,
    pub completed: Vector<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TaskView {
    in_progress: Vec<String>,
    completed: Vec<String>,
}

// Define the default, which automatically initializes the contract
impl Default for TodoList {
    fn default() -> Self {
        Self {
            accounts: UnorderedMap::new(b"a".to_vec()),
        }
    }
}

impl Tasks {
    pub fn new() -> Self {
        Self {
            in_progress: Vector::new(b"i".to_vec()),
            completed: Vector::new(b"c".to_vec()),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl TodoList {
    pub fn get_todo_list(&self, account_id: AccountId) -> TaskView {
        // let account_id = env::predecessor_account_id();
        let tasks_option = self.accounts.get(&account_id);
        match tasks_option {
            Some(tasks) => TaskView {
                in_progress: tasks.in_progress.to_vec(),
                completed: tasks.completed.to_vec(),
            },
            None => {
                // self.accounts.insert(&account_id, &Tasks::new());
                // log!("Initialized new todo list");
                // let tasks = self.accounts.get(&account_id).unwrap();
                TaskView {
                    in_progress: [].to_vec(),
                    completed: [].to_vec(),
                }
            }
        }
    }

    pub fn add_task(&mut self, todo: String) {
        let account_id = env::predecessor_account_id();
        let tasks_option = self.accounts.get(&account_id);
        match tasks_option {
            Some(tasks) => {
                let mut progress_tasks = tasks.in_progress;
                progress_tasks.push(&todo);
                self.accounts.insert(
                    &account_id,
                    &Tasks {
                        in_progress: progress_tasks,
                        ..tasks
                    },
                );
            }
            None => {
                let mut tasks = Tasks::new();
                tasks.in_progress.push(&todo);
                self.accounts.insert(&account_id, &tasks);
            }
        }
    }

    pub fn update_task(&mut self, index: U64, todo: &String) {
        let account_id = env::predecessor_account_id();
        let mut tasks = self.accounts.get(&account_id).unwrap();
        let old_task = tasks.in_progress.replace(index.into(), todo);
        self.accounts.insert(&account_id, &tasks);
        log!("'{}' is replaced with '{}'", old_task, todo);
    }

    pub fn delete_task(&mut self, index: U64) {
        let account_id = env::predecessor_account_id();
        let mut tasks = self.accounts.get(&account_id).unwrap();
        let deleted_task = tasks.in_progress.swap_remove(index.into());
        self.accounts.insert(&account_id, &tasks);
        log!("'{}' is deleted!", deleted_task);
    }

    pub fn check_completed_task(&mut self, index: U64) {
        let account_id = env::predecessor_account_id();
        let mut tasks = self.accounts.get(&account_id).unwrap();
        let completed_task = tasks.in_progress.swap_remove(index.into());
        tasks.completed.push(&completed_task);
        self.accounts.insert(&account_id, &tasks);
        log!("'{}' is completed!", &completed_task);
    }

    pub fn clear_all_completed_tasks(&mut self) {
        let account_id = env::predecessor_account_id();
        let mut tasks = self.accounts.get(&account_id).unwrap();
        tasks.completed.clear();
        self.accounts.insert(&account_id, &tasks);
        log! {"Completed Task List is cleared!"}
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {

    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    const PREDECESSOR: &str = "vinh.test";

    fn init_contract(predecessor: &str) -> TodoList {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor.parse().unwrap());

        testing_env!(builder.build());

        TodoList::default()
    }

    #[test]
    fn get_empty_todo_list() {
        let todo_list = init_contract(PREDECESSOR);
        let get_task = todo_list.get_todo_list(env::predecessor_account_id());
        // assert!(todo_list
        //     .accounts
        //     .contains_key(&env::predecessor_account_id()));
        println!("{:?}", get_task);
        assert!(get_task.completed.is_empty());
        assert!(get_task.in_progress.is_empty());
    }

    #[test]
    fn add_task() {
        let mut todo_list = init_contract(PREDECESSOR);
        let task = "pass add task".to_string();
        todo_list.add_task(task.clone());
        let tasks = todo_list
            .accounts
            .get(&env::predecessor_account_id())
            .unwrap();
        let progress_task = tasks
            .in_progress
            .get(0)
            .unwrap_or_else(|| "hello".to_string());

        assert_eq!(progress_task, task);
    }

    #[test]
    fn update_task() {
        let mut todo_list = init_contract(PREDECESSOR);

        let first_task = "task 0".to_string();
        let second_task = "task 1".to_string();
        let third_task = "task 2".to_string();

        todo_list.add_task(first_task);
        todo_list.add_task(second_task);
        todo_list.add_task(third_task);

        let updated_task = "updated task 1".to_string();
        todo_list.update_task(U64::from(1), &updated_task.clone());

        let tasks = todo_list
            .accounts
            .get(&env::predecessor_account_id())
            .unwrap();
        let updated_second_task = tasks.in_progress.get(1).unwrap();

        assert_eq!(updated_second_task, updated_task);
    }

    #[test]
    fn delete_task() {
        let mut todo_list = init_contract(PREDECESSOR);

        let first_task = "task 0".to_string();
        let second_task = "task 1".to_string();
        let third_task = "task 2".to_string();

        todo_list.add_task(first_task);
        todo_list.add_task(second_task);
        todo_list.add_task(third_task.clone());

        todo_list.delete_task(U64::from(1));

        let tasks = todo_list
            .accounts
            .get(&env::predecessor_account_id())
            .unwrap();
        let progress_task = tasks.in_progress.get(1).unwrap();

        assert_eq!(progress_task, third_task);
        assert_eq!(tasks.in_progress.len(), 2);
    }

    #[test]
    fn check_completed_task() {
        let mut todo_list = init_contract(PREDECESSOR);

        let first_task = "task 0".to_string();
        let second_task = "task 1".to_string();
        let third_task = "task 2".to_string();

        todo_list.add_task(first_task);
        todo_list.add_task(second_task.clone());
        todo_list.add_task(third_task);

        todo_list.check_completed_task(U64::from(1));

        let tasks = todo_list
            .accounts
            .get(&env::predecessor_account_id())
            .unwrap();
        let completed_task = tasks.completed.get(0).unwrap();
        assert_eq!(tasks.in_progress.len(), 2);
        assert_eq!(completed_task, second_task);
        assert_eq!(tasks.completed.len(), 1);
    }

    #[test]
    fn clear_completed_tasks() {
        let mut todo_list = init_contract(PREDECESSOR);

        let first_task = "task 0".to_string();
        let second_task = "task 1".to_string();
        let third_task = "task 2".to_string();

        todo_list.add_task(first_task);
        todo_list.add_task(second_task);
        todo_list.add_task(third_task);

        todo_list.clear_all_completed_tasks();

        let tasks = todo_list
            .accounts
            .get(&env::predecessor_account_id())
            .unwrap();

        assert_eq!(tasks.completed.len(), 0);
    }
}
