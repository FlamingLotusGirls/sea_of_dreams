use std::time::Duration;

use crate::{ELDER_COUNT, Elder};

pub trait Effect {
    fn name(&self) -> String;
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, effect_time: Duration);
}

pub fn get_effect(i: usize) -> Option<Box<dyn Effect>> {
    match i {
        0 => Some(Box::new(DefaultEffect)),
        1 => Some(Box::new(TestEffectA)),
        2 => Some(Box::new(TestEffectB)),
        3 => Some(Box::new(TestEffectC)),
        4 => Some(Box::new(TestEffectD)),
        5 => Some(Box::new(InitialTest)),
        6 => Some(Box::new(FadePairs)),
        7 => Some(Box::new(SolidEffect)),
        8 => Some(Box::new(PoofRing)),
        9 => Some(Box::new(PoofRingWide)),
        10 => Some(Box::new(PoofRingNarrow)),
        11 => Some(Box::new(AllPoof)),
        12 => Some(Box::new(AllPoofWide)),
        13 => Some(Box::new(AllPoofNarrow)),
        _ => None,
    }
}
const PERIOD: f32 = 2.37;

#[derive(Clone, Copy)]
pub struct AllPoofNarrow;
impl Effect for AllPoofNarrow {
    fn render(&mut self, elders: &mut Vec<Elder>, _program_time: Duration, effect_time: Duration) {
        let d = effect_time.as_secs_f32();

        for elder in elders.iter_mut() {
            if d < 0.3 {
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
        let d = effect_time.as_secs_f32();

        for elder in elders.iter_mut() {
            if d < 0.3 {
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
        let d = effect_time.as_secs_f32();

        for elder in elders.iter_mut() {
            if d < 0.3 {
                elder.poofer_both.poof(true);
            } else {
                elder.poofer_both.poof(false);
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
        let d = program_time.as_secs_f32();

        let poof_index = (d * 2.0) as usize % ELDER_COUNT;
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
        let d = program_time.as_secs_f32();

        let poof_index = (d * 2.0) as usize % ELDER_COUNT;
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
        let d = program_time.as_secs_f32();

        let poof_index = (d * 2.0) as usize % ELDER_COUNT;
        for (i, elder) in elders.iter_mut().enumerate() {
            if i == poof_index {
                elder.poofer_both.poof(true);
            } else {
                elder.poofer_both.poof(false);
            }
        }
    }

    fn name(&self) -> String {
        "Poof Ring".into()
    }
}

#[derive(Clone, Copy)]
pub struct DefaultEffect;
impl Effect for DefaultEffect {
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, _effect_time: Duration) {
        let d = program_time.as_secs_f32();
        // let p = 4.0;
        // let prog = ((d % p / p) * 10.);
        let prog = d.sin() * 10.0;
        let width = (d * 1.7).sin() / 2. + 1.;
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
        "Default".into()
    }
}

#[derive(Clone, Copy)]
pub struct SolidEffect;
impl Effect for SolidEffect {
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
pub struct TestEffectA;
impl Effect for TestEffectA {
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, _effect_time: Duration) {
        let len = elders.len();
        let d = program_time.as_secs_f32();
        for (i, elder) in elders.iter_mut().enumerate() {
            let x = ((d % PERIOD) / PERIOD + (i as f32) / len as f32) % 1.;
            elder.crane_light.r = 1. - x;
            elder.crane_light.b = x;
        }
    }

    fn name(&self) -> String {
        "A".into()
    }
}

#[derive(Clone, Copy)]
pub struct TestEffectB;
impl Effect for TestEffectB {
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, _effect_time: Duration) {
        let len = elders.len();
        let d = program_time.as_secs_f32();
        for (i, elder) in elders.iter_mut().enumerate() {
            let x = ((d % PERIOD) / PERIOD + (i as f32) / len as f32) % 1.;
            elder.crane_light.g = 1. - x;
            elder.crane_light.b = x;
        }
    }

    fn name(&self) -> String {
        "B".into()
    }
}

#[derive(Clone, Copy)]
pub struct TestEffectC;
impl Effect for TestEffectC {
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, _effect_time: Duration) {
        let len = elders.len();
        let d = program_time.as_secs_f32();
        for (i, elder) in elders.iter_mut().enumerate() {
            let x = ((d % PERIOD) / PERIOD + (i as f32) / len as f32) % 1.;
            elder.crane_light.r = 1. - x;
            elder.crane_light.g = x;
        }
    }

    fn name(&self) -> String {
        "C".into()
    }
}

#[derive(Clone, Copy)]
pub struct TestEffectD;
impl Effect for TestEffectD {
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, _effect_time: Duration) {
        let len = elders.len();
        let d = program_time.as_secs_f32();
        for (i, elder) in elders.iter_mut().enumerate() {
            let x = ((((d % PERIOD) / PERIOD + (i as f32) / len as f32) * std::f32::consts::TAU)
                .sin()
                + 1.)
                / 2.;
            elder.crane_light.r = 1. - x;
            elder.crane_light.g = x;
        }
    }

    fn name(&self) -> String {
        "D".into()
    }
}

#[derive(Clone, Copy)]
pub struct InitialTest;
impl Effect for InitialTest {
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, _effect_time: Duration) {
        let _len = elders.len();
        let d = program_time.as_secs_f32();
        for (_i, elder) in elders.iter_mut().enumerate() {
            let x = ((((d % PERIOD) / PERIOD) * std::f32::consts::TAU).sin() + 1.) / 2.;
            elder.crane_light.r = (1. - x) / 2.;
            elder.crane_light.g = x;
        }
    }

    fn name(&self) -> String {
        "Main".into()
    }
}

#[derive(Clone, Copy)]
pub struct FadePairs;
impl Effect for FadePairs {
    fn render(&mut self, elders: &mut Vec<Elder>, program_time: Duration, _effect_time: Duration) {
        let _len = elders.len();
        let d = program_time.as_secs_f32();
        let p = 4.0;
        for (i, elder) in elders.iter_mut().skip(60).take(10).enumerate() {
            let lit_i = ((d % p / p) * 10.) as usize;
            let prog = ((d % p / p) * 10.) % 1.;
            let other = 1. - prog;
            let j = (lit_i + 1) % 10;
            if i == lit_i {
                elder.crane_light.r = other * 0.7;
                elder.crane_light.g = other * 0.7;
                elder.crane_light.b = other * 1.;
            } else if i == j {
                elder.crane_light.r = prog * 0.7;
                elder.crane_light.g = prog * 0.7;
                elder.crane_light.b = prog * 1.;
            }
            // let x = ((d % PERIOD) / PERIOD + (i as f32) / len as f32) % 1.;
            // elder.crane_light.r = 1. - x;
            // elder.crane_light.b = x;
        }
    }

    fn name(&self) -> String {
        "FadePairs".into()
    }
}
