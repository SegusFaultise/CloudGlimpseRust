import init from '../../pkg/cloud_glimpse.js'; // d
import { main } from '../../pkg/cloud_glimpse.js'; // Adjust the path as needed

async function runMainWithFile(file) {
    document.getElementById('spinner').style.display = 'block';

    try {
        const arrayBuffer = await file.arrayBuffer();

        main(new Uint8Array(arrayBuffer));
    } 
    //catch (error) {
        //    console.error("Error loading file:", error);
        //} 
    finally {
        document.getElementById('spinner').style.display = 'none';
    }
}

init().then(() => {
    console.log("WASM module loaded and initialized");

    const loadButton = document.getElementById('load-button');
    const test_button = document.getElementById('test-button');

    const fixedFilePath = './points/points.las';

    test_button.addEventListener('click', async () => {
        try {
            const response = await fetch(fixedFilePath);

            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            
            const blob = await response.blob();
            const fixedFile = new File([blob], 'points');

            runMainWithFile(fixedFile);
            moveCanvasToDiv();
            loadTotalPoints();
        } 
        catch (error) {
            console.error('Error loading file:', error);
        }
    });

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
