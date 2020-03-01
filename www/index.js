import * as wasm from "lazor";

function render() {
    console.debug("Calling lazor.render_scene() ...");
    // TODO: provide the canvas' ID to render_scene(): 'result_canvas'
    wasm.render_scene();
}

document.getElementById('render_button').addEventListener("click", render);
