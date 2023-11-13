import init from '/cloud_glimpse/pkg/cloud_glimpse.js';
import { main } from '/cloud_glimpse/pkg/cloud_glimpse.js'; // Adjust the path as needed

init().then(() => {
    console.log("WASM module loaded and initialized");

    // Register the click event handler for the button by its id
    const loadButton = document.getElementById('load-button');
    loadButton.addEventListener('click', async () => {
        const input = document.getElementById('file-input');
        if (!input.files.length) return;

        const file = input.files[0];
        const arrayBuffer = await file.arrayBuffer();

        main(new Uint8Array(arrayBuffer));
    });
}).catch(e => {
    console.error("Failed to initialize the WASM module:", e);
});