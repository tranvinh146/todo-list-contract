import "regenerator-runtime/runtime";
import React from "react";

import "./assets/global.css";

import {
  getTodoList,
  addTask,
  deleteTask,
  updateTask,
  checkCompletedTask,
  clearAllCompletedTasks,
} from "./near-api";
import {
  EducationalText,
  SignInPrompt,
  SignOutButton,
  TodoList,
} from "./ui-components";

export default function App() {
  const [todoList, setTodoList] = React.useState();

  const [uiPleaseWait, setUiPleaseWait] = React.useState(false);

  // Get blockchian state once on component load
  React.useEffect(() => {
    getTodoList()
      .then(setTodoList)
      .catch(alert)
      .finally(() => {
        setUiPleaseWait(false);
      });
  }, []);

  /// If user not signed-in with wallet - show prompt
  if (!window.walletConnection.isSignedIn()) {
    // Sign-in flow will reload the page later
    return <SignInPrompt greeting={valueFromBlockchain} />;
  }

  function addTodo(e) {
    e.preventDefault();
    setUiPleaseWait(true);
    const { todoInput } = e.target.elements;
    addTask(todoInput.value)
      .then(getTodoList)
      .then(setTodoList)
      .catch(alert)
      .finally(() => {
        setUiPleaseWait(false);
      });
  }

  function getList() {
    getTodoList().then(setTodoList);
  }

  return (
    <>
      <SignOutButton accountId={window.accountId} />
      <main className={uiPleaseWait ? "please-wait" : ""}>
        <div>
          <h1>
            Todo List App
            {/* <span className="greeting">{valueFromBlockchain}</span> */}
          </h1>
          <button onClick={getList}>Refresh List</button>
        </div>

        <form onSubmit={addTodo} className="change">
          <label>Add Todo:</label>
          <div>
            <input
              autoComplete="off"
              // defaultValue={valueFromBlockchain}
              id="todoInput"
            />
            <button>
              <span>Add</span>
              <div className="loader"></div>
            </button>
          </div>
        </form>
        <TodoList todoList={todoList} setTodoList={setTodoList} />
        {/* <EducationalText /> */}
      </main>
    </>
  );
}
