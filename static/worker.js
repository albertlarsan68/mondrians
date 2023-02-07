import init, { generate } from './mondrians.js';

var inited = false;

init().then(() => { inited = true; console.debug("Wasm Loaded!"); });

function make_it(data) {
    if (!inited) { setTimeout(make_it, 10, data); return; }
    console.debug("Start Generating");
    console.time("Generating");
    let response = generate(data.seed, data.width, data.height, data.max_depth, data.sep_width, data.min_middle, data.max_middle);
    console.timeEnd("Generating");
    postMessage({ index: data.index, data: response });
}

onmessage = (e) => {
    make_it(e.data)
}
