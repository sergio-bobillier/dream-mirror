const { invoke } = window.__TAURI__.core;

async function exitApp() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  await invoke("exit_app", {});
}

window.addEventListener("DOMContentLoaded", () => {
  const pageWrapper = document.getElementById("page-wrapper");
  const request = new Request("/welcome.html");

  fetch(request).then((response) => {
    if (!response.ok) {
      throw new Error(`Request failed! ${response.status}`)
    }

    return response.text();
  }).then((body) => {
    pageWrapper.innerHTML = body;
  }).catch((error) => {
    console.log(error);
  });

  document.querySelector("#menu-exit").addEventListener("click", (e) => {
    e.preventDefault();
    exitApp();
  });
});
