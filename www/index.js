import * as wasm from "wasm-game-of-life";

const imageInput = document.getElementById("image");
imageInput.addEventListener('change',(e) => {
    const input = e.target;
    const [image] = input.files;
    console.log(image)
    // wasm.greet();
    let fr = new FileReader();
    fr.readAsArrayBuffer(image)
})

