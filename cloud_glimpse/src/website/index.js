import init from '/cloud_glimpse/pkg/cloud_glimpse.js';
import { main } from '/cloud_glimpse/pkg/cloud_glimpse.js'; // Adjust the path as needed

async function runMainWithFile(file) {
    const arrayBuffer = await file.arrayBuffer();
    main(new Uint8Array(arrayBuffer));
        // It's assumed that main or another function from your WASM will handle
        // the canvas creation or manipulation, and that the canvas will end up
        // inside `#canvas-container` or replaces an existing `<canvas>` element.
}

init().then(() => {
    console.log("WASM module loaded and initialized");

    const loadButton = document.getElementById('load-button');
    loadButton.addEventListener('click', () => {
        document.getElementById('file-input').click();
    });

    const fileInput = document.getElementById('file-input');
    fileInput.addEventListener('change', async () => {
        if (!fileInput.files.length) return;

        const file = fileInput.files[0];

        runMainWithFile(file);
        moveCanvasToDiv();
    });
}).catch(e => {
    console.error("Failed to initialize the WASM module:", e);
});

function moveCanvasToDiv() {
    let div = document.getElementById("bevy-canvas");

    // Create a mutation observer to notify us when the canvas is added to the DOM
    const observer = new MutationObserver((mutations, obs) => {
        const canvas = document.querySelector("canvas");
        if (canvas) {
            div.appendChild(canvas);
            console.log("Canvas moved to div.");
            obs.disconnect(); // Stop observing once the canvas is moved
        }
    });

    // Observe the entire document
    observer.observe(document.body, {
        childList: true,  // observe direct children
        subtree: true,    // and lower descendants too
        attributes: false,
        characterData: false,
    });

    console.log("Observer set up to move canvas when available.");
}
