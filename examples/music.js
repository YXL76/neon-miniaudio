const path = require("path");
const NeonMiniaudio = require("../../neon-miniaudio");

const player = new NeonMiniaudio();

player.load(path.resolve(__dirname, "music.wav"));

let flag = false;

setInterval(() => {
  if (flag) {
    player.play();
  } else {
    player.pause();
  }
  flag = !flag;
}, 2000);
