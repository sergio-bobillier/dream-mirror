const { invoke } = window.__TAURI__.core;

async function exitApp() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  await invoke("exit_app", {});
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#menu-exit").addEventListener("click", (e) => {
    e.preventDefault();
    exitApp();
  });
});
