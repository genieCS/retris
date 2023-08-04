import { CursiveWrapper } from "retris";

const canvas = document.getElementById("cursive-wasm-canvas");
canvas.style.display = "block";
canvas.setAttribute("width", "1000");
canvas.setAttribute("height", "1000");
const ctx = canvas.getContext("2d");
ctx.fillStyle = "green";
ctx.fillRect(0, 0, 1000, 1000);

CursiveWrapper.retris();