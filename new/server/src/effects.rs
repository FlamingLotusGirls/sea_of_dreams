use std::time::Duration;

use crate::model::Elder;

pub trait Effect {
    fn name(&self) -> String;
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, effect_time: Duration);
}

pub fn get_ambient_effects() -> Vec<Box<dyn Effect>> {
    vec![
        Box::new(Pseudorandom),
        Box::new(RedToBlue),
        Box::new(GreenToBlue),
        Box::new(FadeRing2Colors),
        Box::new(Unison2Colors),
        Box::new(FadePairs),
        Box::new(Solid),
    ]
}

/**
 * Usually fire but could include LEDs as well. It's up to the effect whether it wants to overwrite
 * the LED value from the current ambient effect.
 */
pub fn get_trigger_effects() -> Vec<Box<dyn Effect>> {
    vec![
        Box::new(PoofRing),
        Box::new(PoofRingWide),
        Box::new(PoofRingNarrow),
        Box::new(AllPoof),
        Box::new(AllPoofWide),
        Box::new(AllPoofNarrow),
    ]
}

#[allow(dead_code)]
pub fn get_effect(i: usize) -> Option<Box<dyn Effect>> {
    get_ambient_effects().into_iter().nth(i)
}

const PERIOD: f32 = 20.;

#[derive(Clone, Copy)]
pub struct AllPoofNarrow;
impl Effect for AllPoofNarrow {
    fn render(&mut self, elders: &mut Vec<Elder>, _program_time: Duration, effect_time: Duration) {
        let t = effect_time.as_secs_f32();

        for elder in elders.iter_mut() {
            if t < 0.3 {
                elder.poofer_narrow.poof(true);
            } else {
                elder.poofer_narrow.poof(false);
            }
        }
    }

    fn name(&self) -> String {
        "All Poof Narrow".into()
    }
}

#[derive(Clone, Copy)]
pub struct AllPoofWide;
impl Effect for AllPoofWide {
    fn render(&mut self, elders: &mut Vec<Elder>, _program_time: Duration, effect_time: Duration) {
        let t = effect_time.as_secs_f32();

        for elder in elders.iter_mut() {
            if t < 0.3 {
                elder.poofer_wide.poof(true);
            } else {
                elder.poofer_wide.poof(false);
            }
        }
    }

    fn name(&self) -> String {
        "All Poof Wide".into()
    }
}

#[derive(Clone, Copy)]
pub struct AllPoof;
impl Effect for AllPoof {
    fn render(&mut self, elders: &mut Vec<Elder>, _program_time: Duration, effect_time: Duration) {
        let t = effect_time.as_secs_f32();

        for elder in elders.iter_mut() {
            if t < 0.3 {
                elder.poofer_wide.poof(true);
                elder.poofer_narrow.poof(true);
            } else {
                elder.poofer_wide.poof(false);
                elder.poofer_narrow.poof(false);
            }
        }
    }

    fn name(&self) -> String {
        "All Poof".into()
    }
}

#[derive(Clone, Copy)]
pub struct PoofRingNarrow;
impl Effect for PoofRingNarrow {
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, _effect_time: Duration) {
        let t = program_time.as_secs_f32();

        let poof_index = (t * 2.0) as usize % elders.len();
        for (i, elder) in elders.iter_mut().enumerate() {
            if i == poof_index {
                elder.poofer_narrow.poof(true);
            } else {
                elder.poofer_narrow.poof(false);
            }
        }
    }

    fn name(&self) -> String {
        "Poof Ring Narrow".into()
    }
}

#[derive(Clone, Copy)]
pub struct PoofRingWide;
impl Effect for PoofRingWide {
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, _effect_time: Duration) {
        let t = program_time.as_secs_f32();

        let poof_index = (t * 2.0) as usize % elders.len();
        for (i, elder) in elders.iter_mut().enumerate() {
            if i == poof_index {
                elder.poofer_wide.poof(true);
            } else {
                elder.poofer_wide.poof(false);
            }
        }
    }

    fn name(&self) -> String {
        "Poof Ring Wide".into()
    }
}

#[derive(Clone, Copy)]
pub struct PoofRing;
impl Effect for PoofRing {
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, _effect_time: Duration) {
        let t = program_time.as_secs_f32();

        let poof_index = (t * 2.0) as usize % elders.len();
        for (i, elder) in elders.iter_mut().enumerate() {
            if i == poof_index {
                elder.poofer_wide.poof(true);
                elder.poofer_narrow.poof(true);
            } else {
                elder.poofer_wide.poof(false);
                elder.poofer_narrow.poof(false);
            }
        }
    }

    fn name(&self) -> String {
        "Poof Ring".into()
    }
}

#[derive(Clone, Copy)]
pub struct Pseudorandom;
impl Effect for Pseudorandom {
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, _effect_time: Duration) {
        let t = program_time.as_secs_f32() * 0.08;
        // let p = 4.0;
        // let prog = ((t % p / p) * 10.);
        let prog = t.sin() * 10.0;
        let width = (t * 1.7).sin() / 2. + 1.;
        let len = elders.len() as i32;
        // for (i, elder) in elders.iter_mut().skip(50).take(30).enumerate() {
        for (i, elder) in elders.iter_mut().enumerate() {
            let b = (((prog + (i as i32 - len / 2) as f32) * width).sin() + 1.) / 2.;
            elder.crane_light.r = 0.7 * b;
            elder.crane_light.g = 0.7 * b;
            elder.crane_light.b = 1. * b;
        }
    }

    fn name(&self) -> String {
        "Pseudorandom".into()
    }
}

#[derive(Clone, Copy)]
pub struct Solid;
impl Effect for Solid {
    fn render(&mut self, elders: &mut Vec<Elder>, _program_time: Duration, _effect_time: Duration) {
        for (_i, elder) in elders.iter_mut().enumerate() {
            elder.crane_light.r = 0.7;
            elder.crane_light.g = 0.7;
            elder.crane_light.b = 1.;
        }
    }

    fn name(&self) -> String {
        "Solid".into()
    }
}

#[derive(Clone, Copy)]
pub struct RedToBlue;
impl Effect for RedToBlue {
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, _effect_time: Duration) {
        let len = elders.len();
        let t = program_time.as_secs_f32();
        for (i, elder) in elders.iter_mut().enumerate() {
            let x = ((t % PERIOD) / PERIOD + (i as f32) / len as f32) % 1.;
            elder.crane_light.r = 1. - x;
            elder.crane_light.b = x;
        }
    }

    fn name(&self) -> String {
        "Red to Blue".into()
    }
}

#[derive(Clone, Copy)]
pub struct GreenToBlue;
impl Effect for GreenToBlue {
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, _effect_time: Duration) {
        let len = elders.len();
        let t = program_time.as_secs_f32();
        for (i, elder) in elders.iter_mut().enumerate() {
            let x = ((t % PERIOD) / PERIOD + (i as f32) / len as f32) % 1.;
            elder.crane_light.g = 1. - x;
            elder.crane_light.b = x;
        }
    }

    fn name(&self) -> String {
        "Green to Blue".into()
    }
}

#[derive(Clone, Copy)]
pub struct FadeRing2Colors;
impl Effect for FadeRing2Colors {
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, _effect_time: Duration) {
        let len = elders.len() as f32;
        let t = program_time.as_secs_f32();
        for (i, elder) in elders.iter_mut().enumerate() {
            let t = ((((t % PERIOD) / PERIOD + (i as f32) / len) * std::f32::consts::TAU).sin()
                + 1.)
                / 2.;
            elder.crane_light.r = 1. - t;
            elder.crane_light.g = t;
            elder.crane_light.b = t.max(1. - t);
        }
    }

    fn name(&self) -> String {
        "Fade Ring 2 Colors".into()
    }
}

#[derive(Clone, Copy)]
pub struct Unison2Colors;
impl Effect for Unison2Colors {
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, _effect_time: Duration) {
        let _len = elders.len();
        let t = program_time.as_secs_f32();
        for (_i, elder) in elders.iter_mut().enumerate() {
            let x = ((((t % PERIOD) / PERIOD) * std::f32::consts::TAU).sin() + 1.) / 2.;
            elder.crane_light.r = 1. - x;
            elder.crane_light.g = x;
            elder.crane_light.b = x.max(1. - x);
        }
    }

    fn name(&self) -> String {
        "Unison 2 Colors".into()
    }
}

#[derive(Clone, Copy)]
pub struct FadePairs;
impl Effect for FadePairs {
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, _effect_time: Duration) {
        let len = elders.len();
        let t = program_time.as_secs_f32();

        let fade_in_index = ((t % PERIOD / PERIOD) * 10.) as usize;
        let fade_out_index = (fade_in_index + 1) % len;

        let fade_in_brightness = ((t % PERIOD / PERIOD) * 10.) % 1.;
        let fade_out_brightness = 1. - fade_in_brightness;

        for (i, elder) in elders.iter_mut().enumerate() {
            if i == fade_in_index {
                elder.crane_light.r = fade_out_brightness * 0.7;
                elder.crane_light.g = fade_out_brightness * 0.7;
                elder.crane_light.b = fade_out_brightness * 1.;
            } else if i == fade_out_index {
                elder.crane_light.r = fade_in_brightness * 0.7;
                elder.crane_light.g = fade_in_brightness * 0.7;
                elder.crane_light.b = fade_in_brightness * 1.;
            }
        }
    }

    fn name(&self) -> String {
        "Fade Pairs".into()
    }
}
