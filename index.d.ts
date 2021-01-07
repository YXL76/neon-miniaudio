declare module "neon-miniaudio" {
  export default class NeonMiniaudio {
    isPlaying(): boolean;
    load(url: string): boolean;
    pause(): void;
    play(): boolean;
    volume(level: number): void;
    stop(): void;
  }
}
