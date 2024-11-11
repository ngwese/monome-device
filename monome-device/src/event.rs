//
// Copyright (c) 2024 Greg Wuller
//
// SPDX-License-Identifier: MIT
//

#[derive(Debug)]
pub enum Event {
    ButtonPress { x: u32, y: u32 },
    ButtonLift { x: u32, y: u32 },
    EncoderDelta { number: u32, delta: i32 },
    EncoderPress { number: u32 },
    EncoderLift { number: u32 },
    Tilt { sensor: u32, x: i32, y: i32, z: i32 },
}