import init from '../../pkg/cloud_glimpse.js'; // Path for gh pages
import { main } from '../../pkg/cloud_glimpse.js'; // Path for gh pages

document.addEventListener('DOMContentLoaded', function() {
    function resizeBevyCanvas() {
        var bevyCanvas = document.querySelector('canvas');

        if (!bevyCanvas) {
            console.error('Canvas element not found.');
            return;
        }

        var originalWidth = parseInt(bevyCanvas.getAttribute('width'));
        var originalHeight = parseInt(bevyCanvas.getAttribute('height'));
        var aspectRatio = originalWidth / originalHeight;

        var containerWidth = bevyCanvas.parentElement.clientWidth;

        bevyCanvas.style.width = containerWidth + 'px'; // Set width to the container width
        bevyCanvas.style.height = (containerWidth / aspectRatio) + 'px'; // Calculate height based on aspect ratioum height if needed
    }

    resizeBevyCanvas();

    window.addEventListener('resize', function() {
        resizeBevyCanvas();
    });
});

async function runMainWithFile(file) {
    document.getElementById('spinner').style.display = 'block';

    try {
        const arrayBuffer = await file.arrayBuffer();

        main(new Uint8Array(arrayBuffer));
    } 
    catch (error) {
        console.error("Error loading file:", error);
    } 
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
            const blob = await response.blob();
            const fixedFile = new File([blob], 'points');

            const bevyCanvas = document.querySelector("canvas");

            if(!response.ok) {
                throw new Error('Network response was not ok');
            }
            if(bevyCanvas != null) {
                if(confirm("CloudGlimpse is already running? do you wish to restart?")) {

                    location.reload();
                    return false;
                }
            }
            else {
                runMainWithFile(fixedFile);
                moveCanvasToDiv();
            }
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
        const bevyCanvas = document.querySelector("canvas");

        if(!fileInput.files.length) return;
        if(bevyCanvas != null) {
            alert("CloudGlimpse is already running!");
            
            location.reload();
            return false;
        }

        const file = fileInput.files[0];

        runMainWithFile(file);
        moveCanvasToDiv();
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
