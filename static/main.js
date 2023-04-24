const form = document.querySelector("form");
const output = document.querySelector("img#output");
const submitButton = document.querySelector("input#generate");
const ramdomFill = document.querySelector("button#randomFill");
const seed = document.querySelector("input#seed");
var generating = false;
var toGenerate = null;
var erroring = false;
submitButton.disabled = true;
submitButton.value = "Chargement en cours...";

var worker = new Worker("/{hash}/worker.js", { type: "module" });

function workerOnError(error) {
    console.log(`Worker error: ${error.message}`);
    submitButton.disabled = false;
    submitButton.value = "Erreur lors de la génération, modifiez les paramètres et réessayez";
    erroring = true;
    toGenerate = null;
    generating = false;
    setTimeout(() => { worker.terminate(); worker = new Worker("{hash}/worker.js", { type: "module" }); worker.onerror = workerOnError; worker.onmessage = workerOnMessage; erroring = false; }, 1);
    throw error;
}

function workerOnMessage(event) {
    if (toGenerate != null) {
        worker.postMessage(toGenerate);
        toGenerate = null;
    } else {
        generating = false;
        submitButton.disabled = false;
        submitButton.value = "Générer";
    }
    output.src = ("data:image/png;base64," + event.data.data);
    output.hidden = false;
};

worker.onerror = workerOnError;
worker.onmessage = workerOnMessage;

randomFill.onclick = (e) => {
    const random = (length = 20) => {
        // Declare all characters
        let chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';

        // Pick characers randomly
        let str = '';
        for (let i = 0; i < length; i++) {
            str += chars.charAt(Math.floor(Math.random() * chars.length));
        }

        return str;

    };
    seed.value = random();
    main();
};

form.onsubmit = (e) => {
    e.preventDefault();
    main();
}

form.oninput = (e) => {
    main();
};

function generate(seed, width, height, max_depth, sep_width, min_middle, max_middle) {
    if (generating) {
        toGenerate = { seed: seed, width: width, height: height, max_depth: max_depth, sep_width: sep_width, min_middle: min_middle, max_middle: max_middle };
    } else {
        worker.postMessage({ seed: seed, width: width, height: height, max_depth: max_depth, sep_width: sep_width, min_middle: min_middle, max_middle: max_middle });
        generating = true;
    }

}

function main() {
    if (!form.checkValidity()) {
        return;
    }
    if (erroring) {
        setTimeout(main, 10);
    }
    submitButton.disabled = true;
    submitButton.value = "Génération en cours...";
    generate(Uint8Array.from(document.getElementById("seed").value), Number.parseInt(document.getElementById("width").value), Number.parseInt(document.getElementById("height").value), Number.parseInt(document.getElementById("max-depth").value), Number.parseInt(document.getElementById("sep-width").value), Number.parseInt(document.getElementById("min-middle").value), Number.parseInt(document.getElementById("max-middle").value));
}

main()
submitButton.value = "Chargement en cours...";
