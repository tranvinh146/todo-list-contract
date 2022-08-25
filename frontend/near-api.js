import { connect, Contract, keyStores, WalletConnection } from "near-api-js";
import { getConfig } from "./near-config";

const nearConfig = getConfig(process.env.NODE_ENV || "development");

// Initialize contract & set global variables
export async function initContract() {
  // Initialize connection to the NEAR testnet
  const near = await connect(
    Object.assign(
      { deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } },
      nearConfig
    )
  );

  // Initializing Wallet based Account. It can work with NEAR testnet wallet that
  // is hosted at https://wallet.testnet.near.org
  window.walletConnection = new WalletConnection(near);

  // Getting the Account ID. If still unauthorized, it's just empty string
  window.accountId = window.walletConnection.getAccountId();

  // Initializing our contract APIs by contract name and configuration
  window.contract = await new Contract(
    window.walletConnection.account(),
    nearConfig.contractName,
    {
      // View methods are read only. They don't modify the state, but usually return some value.
      viewMethods: ["get_todo_list"],
      // Change methods can modify the state. But you don't receive the returned value when called.
      changeMethods: [
        "add_task",
        "update_task",
        "delete_task",
        "check_completed_task",
        "clear_all_completed_tasks",
      ],
    }
  );
}

export function signOutNearWallet() {
  window.walletConnection.signOut();
  // reload page
  window.location.replace(window.location.origin + window.location.pathname);
}

export function signInWithNearWallet() {
  // Allow the current app to make calls to the specified contract on the
  // user's behalf.
  // This works by creating a new access key for the user's account and storing
  // the private key in localStorage.
  window.walletConnection.requestSignIn(nearConfig.contractName);
}

export async function getTodoList() {
  let todoList = await window.contract.get_todo_list({
    account_id: window.accountId,
  });
  return todoList;
}

export async function addTask(todo) {
  let response = await window.contract.add_task({
    args: { todo: todo },
  });
  return response;
}

export async function updateTask(index, content) {
  let response = await window.contract.update_task({
    args: { index: index.toString(), todo: content },
  });
  return response;
}

export async function deleteTask(index) {
  let response = await window.contract.delete_task({
    args: { index: index.toString() },
  });
  return response;
}

export async function checkCompletedTask(index) {
  let response = await window.contract.check_completed_task({
    args: { index: index.toString() },
  });
  return response;
}

export async function clearAllCompletedTasks(index, todo) {
  let response = await window.contract.clear_all_completed_tasks();
  return response;
}
