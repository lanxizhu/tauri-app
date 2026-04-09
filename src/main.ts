import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useSplashScreen } from "./useSplashScreen";

listen<string>("update-ready", (event) => {
  console.log(`update ready: ${event.payload}`);
  document.querySelector(".restart-tip")?.classList.toggle("show");
});

listen<string>("update-not-available", (event) => {
  console.log(`update not available: ${event.payload}`);
  document.querySelector(".no-update-tip")?.classList.toggle("show");

  setTimeout(() => {
    document.querySelector(".no-update-tip")?.classList.toggle("show");
  }, 3000);
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

  document
    .querySelector("#toggle-update-tips")
    ?.addEventListener("click", (e) => {
      e.preventDefault();
      document.querySelector(".restart-tip")?.classList.toggle("show");
      document.querySelector(".no-update-tip")?.classList.toggle("show");
    });

  document.querySelector("#restart")?.addEventListener("click", (e) => {
    e.preventDefault();
    relaunch();
  });

  document.querySelector("#later")?.addEventListener("click", (e) => {
    e.preventDefault();
    document.querySelector(".restart-tip")?.classList.remove("show");
  });

  document
    .querySelector("#check-updates-later")
    ?.addEventListener("click", (e) => {
      e.preventDefault();
      document.querySelector(".no-update-tip")?.classList.remove("show");
    });

  if (!import.meta.env.DEV) {
    document.getElementById("toggle-update-tips")!.style.display = "none";
  }
});

useSplashScreen();
