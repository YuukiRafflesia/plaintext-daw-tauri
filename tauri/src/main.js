const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let projInputEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function syscall() {
  await invoke('syscall_test', {});
}

async function renderProject(backend) {
  await invoke('render_project', { path: projInputEl.value, backend: backend });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  projInputEl = document.querySelector("#project-file-input");

  document
    .querySelector("#greet-button")
    .addEventListener("click", () => greet());
  document
    .querySelector("#syscall-button")
    .addEventListener("click", () => syscall());
  document
    .querySelector("#project-render-button-rs")
    .addEventListener("click", () => renderProject("rs"));
  document
    .querySelector("#project-render-button-py")
    .addEventListener("click", () => renderProject("py"));
});
