use std::time::Duration;

use crate::Pixel;

pub trait Effect {
    fn name(&self) -> String;
    fn render(&mut self, target: &mut Vec<Pixel>, time: Duration);
}

pub fn get_effect(i: usize) -> Option<Box<dyn Effect>> {
    match i {
        0 => Some(Box::new(TestA)),
        1 => Some(Box::new(TestEffectA)),
        2 => Some(Box::new(TestEffectB)),
        3 => Some(Box::new(TestEffectC)),
        4 => Some(Box::new(TestEffectD)),
        5 => Some(Box::new(InitialTest)),
        6 => Some(Box::new(FadePairs)),
        7 => Some(Box::new(ColorTest)),
        _ => None,
    }
}
const PERIOD: f32 = 2.37;

#[derive(Clone, Copy)]
pub struct TestA;
impl Effect for TestA {
    fn render(&mut self, target: &mut Vec<Pixel>, time: Duration) {
        let d = time.as_secs_f32();
        // let p = 4.0;
        // let prog = ((d % p / p) * 10.);
        let prog = d.sin() * 10.0;
        let width = (d * 1.7).sin() / 2. + 1.;
        let len = target.len() as i32;
        // for (i, pixel) in target.iter_mut().skip(50).take(30).enumerate() {
        for (i, pixel) in target.iter_mut().enumerate() {
            let b = (((prog + (i as i32 - len / 2) as f32) * width).sin() + 1.) / 2.;
            pixel.r = 0.7 * b;
            pixel.g = 0.7 * b;
            pixel.b = 1. * b;
        }
    }

    fn name(&self) -> String {
        "AA".into()
    }
}

#[derive(Clone, Copy)]
pub struct ColorTest;
impl Effect for ColorTest {
    fn render(&mut self, target: &mut Vec<Pixel>, _time: Duration) {
        for (_i, pixel) in target.iter_mut().enumerate() {
            pixel.r = 0.7;
            pixel.g = 0.7;
            pixel.b = 1.;
        }
    }

    fn name(&self) -> String {
        "Solid".into()
    }
}

#[derive(Clone, Copy)]
pub struct TestEffectA;
impl Effect for TestEffectA {
    fn render(&mut self, target: &mut Vec<Pixel>, time: Duration) {
        let len = target.len();
        let d = time.as_secs_f32();
        for (i, pixel) in target.iter_mut().enumerate() {
            let x = ((d % PERIOD) / PERIOD + (i as f32) / len as f32) % 1.;
            pixel.r = 1. - x;
            pixel.b = x;
        }
    }

    fn name(&self) -> String {
        "A".into()
    }
}

#[derive(Clone, Copy)]
pub struct TestEffectB;
impl Effect for TestEffectB {
    fn render(&mut self, target: &mut Vec<Pixel>, time: Duration) {
        let len = target.len();
        let d = time.as_secs_f32();
        for (i, pixel) in target.iter_mut().enumerate() {
            let x = ((d % PERIOD) / PERIOD + (i as f32) / len as f32) % 1.;
            pixel.g = 1. - x;
            pixel.b = x;
        }
    }

    fn name(&self) -> String {
        "B".into()
    }
}

#[derive(Clone, Copy)]
pub struct TestEffectC;
impl Effect for TestEffectC {
    fn render(&mut self, target: &mut Vec<Pixel>, time: Duration) {
        let len = target.len();
        let d = time.as_secs_f32();
        for (i, pixel) in target.iter_mut().enumerate() {
            let x = ((d % PERIOD) / PERIOD + (i as f32) / len as f32) % 1.;
            pixel.r = 1. - x;
            pixel.g = x;
        }
    }

    fn name(&self) -> String {
        "C".into()
    }
}

#[derive(Clone, Copy)]
pub struct TestEffectD;
impl Effect for TestEffectD {
    fn render(&mut self, target: &mut Vec<Pixel>, time: Duration) {
        let len = target.len();
        let d = time.as_secs_f32();
        for (i, pixel) in target.iter_mut().enumerate() {
            let x = ((((d % PERIOD) / PERIOD + (i as f32) / len as f32) * std::f32::consts::TAU)
                .sin()
                + 1.)
                / 2.;
            pixel.r = 1. - x;
            pixel.g = x;
        }
    }

    fn name(&self) -> String {
        "D".into()
    }
}

#[derive(Clone, Copy)]
pub struct InitialTest;
impl Effect for InitialTest {
    fn render(&mut self, target: &mut Vec<Pixel>, time: Duration) {
        let _len = target.len();
        let d = time.as_secs_f32();
        for (_i, pixel) in target.iter_mut().enumerate() {
            let x = ((((d % PERIOD) / PERIOD) * std::f32::consts::TAU).sin() + 1.) / 2.;
            pixel.r = (1. - x) / 2.;
            pixel.g = x;
        }
    }

    fn name(&self) -> String {
        "Main".into()
    }
}

#[derive(Clone, Copy)]
pub struct FadePairs;
impl Effect for FadePairs {
    fn render(&mut self, target: &mut Vec<Pixel>, time: Duration) {
        let _len = target.len();
        let d = time.as_secs_f32();
        let p = 4.0;
        for (i, pixel) in target.iter_mut().skip(60).take(10).enumerate() {
            let lit_i = ((d % p / p) * 10.) as usize;
            let prog = ((d % p / p) * 10.) % 1.;
            let other = 1. - prog;
            let j = (lit_i + 1) % 10;
            if i == lit_i {
                pixel.r = other * 0.7;
                pixel.g = other * 0.7;
                pixel.b = other * 1.;
            } else if i == j {
                pixel.r = prog * 0.7;
                pixel.g = prog * 0.7;
                pixel.b = prog * 1.;
            }
            // let x = ((d % PERIOD) / PERIOD + (i as f32) / len as f32) % 1.;
            // pixel.r = 1. - x;
            // pixel.b = x;
        }
    }

    fn name(&self) -> String {
        "FadePairs".into()
    }
}
