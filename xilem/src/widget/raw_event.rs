use druid_shell::{
    kurbo::{Point, Vec2},
    Code, KbKey, KeyState, Location, Modifiers, MouseButton, MouseButtons,
};

// Copyright 2022 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Debug, Clone)]
pub enum RawEvent {
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    MouseMove(MouseEvent),
    MouseWheel(MouseEvent),
}

#[derive(Debug, Clone)]
pub struct MouseEvent {
    /// The position of the mouse in the coordinate space of the receiver.
    pub pos: Point,
    /// The position of the mose in the window coordinate space.
    pub window_pos: Point,
    pub buttons: MouseButtons,
    pub mods: Modifiers,
    pub count: u8,
    pub focus: bool,
    pub button: MouseButton,
    pub wheel_delta: Vec2,
}

#[derive(Clone, Debug)]
pub struct KeyEvent {
    pub state: KeyState,
    pub key: KbKey,
    pub code: Code,
    pub location: Location,
    pub mods: Modifiers,
    pub repeat: bool,
    pub is_composing: bool,
}

#[derive(Debug)]
pub enum LifeCycle {
    HotChanged(bool),
}

impl<'a> From<&'a druid_shell::MouseEvent> for MouseEvent {
    fn from(src: &druid_shell::MouseEvent) -> MouseEvent {
        let druid_shell::MouseEvent {
            pos,
            buttons,
            mods,
            count,
            focus,
            button,
            wheel_delta,
        } = src;
        MouseEvent {
            pos: *pos,
            window_pos: *pos,
            buttons: *buttons,
            mods: *mods,
            count: *count,
            focus: *focus,
            button: *button,
            wheel_delta: *wheel_delta,
        }
    }
}

impl From<druid_shell::KeyEvent> for KeyEvent {
    fn from(src: druid_shell::KeyEvent) -> KeyEvent {
        let druid_shell::KeyEvent {
            state,
            key,
            code,
            location,
            mods,
            repeat,
            is_composing,
            ..
        } = src;
        KeyEvent {
            state,
            key,
            code,
            location,
            mods,
            repeat,
            is_composing,
        }
    }
}
