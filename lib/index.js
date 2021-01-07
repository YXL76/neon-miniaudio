const {
  playerIsPlaying,
  playerLoad,
  playerNew,
  playerPause,
  playerPlay,
  playerVolume,
  playerStop,
} = require("../index.node");

class NeonMiniaudio {
  constructor() {
    this.player = playerNew();
  }

  isPlaying() {
    return playerIsPlaying(this.player);
  }

  load(url) {
    return playerLoad(this.player, url);
  }

  pause() {
    playerPause(this.player);
  }

  play() {
    playerPlay(this.player);
  }

  volume(level) {
    return playerVolume(this.player, level);
  }

  stop() {
    playerStop(this.player);
  }
}

module.exports = NeonMiniaudio;
