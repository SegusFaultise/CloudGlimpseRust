import init from '../cloud_glimpse/pkg/cloud_glimpse.js'; // d
import { main } from '../cloud_glimpse/pkg/cloud_glimpse.js'; // Adjust the path as needed

async function runMainWithFile(file) {
    const arrayBuffer = await file.arrayBuffer();

    main(new Uint8Array(arrayBuffer));
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
        loadTotalPoints();
    });
}).catch(e => {
    console.error("Failed to initialize the WASM module:", e);
});

function moveCanvasToDiv() {
    let div = document.getElementById("bevy-canvas");

    const observer = new MutationObserver((mutations, obs) => {
        const canvas = document.querySelector("canvas");
        if (canvas) {
            div.appendChild(canvas);
            console.log("Canvas moved to div.");
            obs.disconnect();
        }
    });

    observer.observe(document.body, {
        childList: true,  
        subtree: true,  
        attributes: false,
        characterData: false,
    });
    console.log("Observer set up to move canvas when available.");
}
