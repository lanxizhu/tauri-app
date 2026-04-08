import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useSplashScreen } from "./useSplashScreen";

listen<string>("update-ready", (event) => {
  console.log(`update ready: ${event.payload}`);
  document.querySelector(".restart-tip")?.classList.toggle("show");
});

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

function relaunch() {
  // Learn more about relaunching the app, either with or without arguments, at https://tauri.app/develop/api/js/modules/app/#relaunch
  invoke("restart", { args: ["--test"] });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  document.querySelector("#check-updates")?.addEventListener("click", (e) => {
    e.preventDefault();
    document.querySelector(".restart-tip")?.classList.toggle("show");
  });

  document.querySelector("#restart")?.addEventListener("click", (e) => {
    e.preventDefault();
    relaunch();
  });

  document.querySelector("#later")?.addEventListener("click", (e) => {
    e.preventDefault();
    document.querySelector(".restart-tip")?.classList.remove("show");
  });
});

useSplashScreen();
