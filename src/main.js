const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

async function greet() {
// 1. Get the data from Rust
const songs = await invoke("search_music", { query: greetInputEl.value });

// 2. Clear out the message area so old results disappear
greetMsgEl.innerHTML = ""; 

// 3. Loop through each song and add it to the page
songs.forEach((song) => {
  const songDiv = document.createElement("div");
  songDiv.innerHTML = `<strong>${song.title}</strong> - ${song.artist}`;
  greetMsgEl.appendChild(songDiv);
});
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
