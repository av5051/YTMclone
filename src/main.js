const { invoke } = window.__TAURI__.core;

window.search = async () => {
    // 1. Correct the IDs to match index.html
    const queryInput = document.querySelector("#search-input");
    const listContainer = document.querySelector("#music-results");

    // 2. Call Rust (The parameter 'query' matches your Rust code)
    const songs = await invoke("search_music", { query: queryInput.value });

    // 3. Clear and Build
    listContainer.innerHTML = "";
    songs.forEach((song) => {
        const div = document.createElement("div");
        div.className = "song-card";
        div.innerHTML = `
            <div>
                <strong>${song.title}</strong><br>
                <small>${song.artist}</small>
            </div>
            <button onclick="window.playSong('${song.id}')">Play</button>
        `;
        listContainer.appendChild(div);
    });
};

window.playSong = async (id) => {
    const streamUrl = await invoke("get_streaming_url", { id: id });
    let player = new Audio(streamUrl);
    player.play();
};