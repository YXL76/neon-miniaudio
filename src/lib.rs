use neon::prelude::*;
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

pub struct Player {
    device: miniaudio::Device,
    playing: Arc<Mutex<bool>>,
}

impl Finalize for Player {}

impl Player {
    #[inline]
    fn new() -> Self {
        let config = miniaudio::DeviceConfig::new(miniaudio::DeviceType::Playback);
        let device = miniaudio::Device::new(None, &config).unwrap();
        Player {
            device,
            playing: Arc::new(Mutex::new(false)),
        }
    }

    #[inline]
    pub fn load(&mut self, url: String) -> bool {
        if let Ok(mut decoder) = miniaudio::Decoder::from_file(url, None) {
            let mut config = miniaudio::DeviceConfig::new(miniaudio::DeviceType::Playback);
            config.playback_mut().set_format(decoder.output_format());
            config
                .playback_mut()
                .set_channels(decoder.output_channels());
            config.set_sample_rate(decoder.output_sample_rate());
            if let Ok(device) = miniaudio::Device::new(None, &config) {
                self.device = device;
                let playing = Arc::clone(&self.playing);
                self.device
                    .set_data_callback(move |_device, output, _frames| {
                        let mut playing = playing.lock().unwrap();
                        if *playing {
                            let frames = decoder.read_pcm_frames(output);
                            if frames == 0 {
                                *playing = false;
                            }
                        }
                    });
                if let Ok(_) = self.device.start() {
                    self.play();
                    return true;
                }
            }
        }
        false
    }

    pub fn play(&self) {
        *self.playing.lock().unwrap() = true
    }

    pub fn pause(&self) {
        *self.playing.lock().unwrap() = false
    }

    pub fn stop(&self) {
        *self.playing.lock().unwrap() = false;
        self.device.stop().unwrap_or(());
    }

    pub fn set_volume(&self, volume: f32) {
        self.device.set_master_volume(volume).unwrap_or(());
    }

    pub fn is_playing(&self) -> bool {
        *self.playing.lock().unwrap()
    }
}

pub fn player_new(mut cx: FunctionContext) -> JsResult<JsValue> {
    let player = RefCell::new(Player::new());

    Ok(cx.boxed(player).upcast())
}

pub fn player_load(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let player = cx.argument::<JsBox<RefCell<Player>>>(0)?;
    let url = cx.argument::<JsString>(1)?.value(&mut cx);
    let res = player.borrow_mut().load(url);

    Ok(cx.boolean(res))
}

pub fn player_play(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let player = cx.argument::<JsBox<RefCell<Player>>>(0)?;
    player.borrow().play();

    Ok(cx.undefined())
}

pub fn player_pause(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let player = cx.argument::<JsBox<RefCell<Player>>>(0)?;
    player.borrow().pause();

    Ok(cx.undefined())
}

pub fn player_stop(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let player = cx.argument::<JsBox<RefCell<Player>>>(0)?;
    player.borrow().stop();

    Ok(cx.undefined())
}
pub fn player_volume(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let player = cx.argument::<JsBox<RefCell<Player>>>(0)?;
    let level = cx.argument::<JsNumber>(1)?.value(&mut cx) / 100.0;
    player.borrow().set_volume(level as f32);

    Ok(cx.undefined())
}

pub fn player_is_playing(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let player = cx.argument::<JsBox<RefCell<Player>>>(0)?;
    let res = player.borrow().is_playing();

    Ok(cx.boolean(res))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("playerIsPlaying", player_is_playing)?;
    cx.export_function("playerLoad", player_load)?;
    cx.export_function("playerNew", player_new)?;
    cx.export_function("playerPause", player_pause)?;
    cx.export_function("playerPlay", player_play)?;
    cx.export_function("playerVolume", player_volume)?;
    cx.export_function("playerStop", player_stop)?;

    Ok(())
}
