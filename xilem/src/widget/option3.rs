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

//! A simple scroll view.
//!
//! There's a lot more functionality in the Druid version, including
//! control over scrolling axes, ability to scroll to content, etc.

use druid_shell::{
    kurbo::{Affine, Rect, Size, Vec2},
    piet::RenderContext,
};

use crate::Widget;

use super::{
    contexts::LifeCycleCx, EventCx, LayoutCx, LifeCycle, PaintCx, Pod, PreparePaintCx, RawEvent,
    UpdateCx,
};

pub struct OptionWidget {
    child: Option<Pod>,
}

impl OptionWidget {
    pub fn new(child: Option<impl Widget + 'static>) -> Self {
        OptionWidget {
            child: child.map(|child| Pod::new(child)),
        }
    }

    pub fn child_mut(&mut self) -> Option<&mut Pod> {
        self.child.as_mut()
    }

    pub fn set_child(&mut self, child: Option<impl Widget + 'static>) {
        self.child = child.map(|child| Pod::new(child));
    }
}

impl Widget for OptionWidget {
    fn event(&mut self, _cx: &mut EventCx, _event: &RawEvent) {}

    fn lifecycle(&mut self, cx: &mut LifeCycleCx, event: &LifeCycle) {
        if let Some(child) = &mut self.child {
            child.lifecycle(cx, event);
        }
    }

    fn update(&mut self, cx: &mut UpdateCx) {
        if let Some(child) = &mut self.child {
            child.update(cx);
        }
        cx.request_layout();
    }

    fn measure(&mut self, cx: &mut LayoutCx) -> (Size, Size) {
        if let Some(child) = &mut self.child {
            let _ = child.measure(cx);
        }
        (Size::ZERO, Size::new(1e9, 1e9))
    }

    fn layout(&mut self, cx: &mut LayoutCx, proposed_size: Size) -> Size {
        if let Some(child) = &mut self.child {
            child.layout(cx, proposed_size)
        } else {
            Size::ZERO
        }
    }

    fn prepare_paint(&mut self, cx: &mut PreparePaintCx, visible: Rect) {
        if let Some(child) = &mut self.child {
            child.prepare_paint(cx, visible);
        }
    }

    fn paint(&mut self, cx: &mut PaintCx) {
        cx.with_save(|cx| {
            if let Some(child) = &mut self.child {
                let size = cx.size();
                cx.clip(size.to_rect());
                child.paint_raw(cx);
            }
        });
    }
}
