# neon-miniaudio

> Miniaudio for Node.js

`neon-miniaudio` allows you to play music files using javascript

## Installation

```bash
npm install neon-miniaudio
yarn add neon-miniaudio
```

## Usage

```javascript
const NeonMiniaudio = require("../../neon-miniaudio");

const player = new NeonMiniaudio();

player.load("path/to/test.wav");
```

## APIs

```typescript
.isPlaying(): boolean;          // check if it is playing
.load(url: string): boolean;    // load music file (return true if loading succeeded)
.pause(): void;                 // pause playback
.play(): void;                  // resume playback
.volume(level: number): void;   // set player volume (0-100)
.stop(): void;                  // stop playback
```
