// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
const { invoke } = window.__TAURI__.tauri;

let element = document.getElementById("genbutton");

element.addEventListener("click", () => {
    let fileName = document.getElementById("input").value;
    invoke("parse_cpp_file", { fileName })
    .then(generate_flowchart)
    .catch(report_error);
})

function generate_flowchart(parse_vector) {
    console.log(parse_vector)
}

function report_error(error) {
    let { CodeErrors, NoMain } = error;
    if (CodeErrors) {
        console.warn("Your code has errors!");
        CodeErrors.errors.forEach(element => {
            console.warn(element) 
        });
    } else if (NoMain) {
        console.warn("There is no main function in your code!");
    }
}