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
    kurbo::{Point, Size},
    piet::{Color, RenderContext, StrokeStyle},
};

use crate::Widget;

use super::{
    contexts::LifeCycleCx, EventCx, LayoutCx, LifeCycle, PaintCx, Pod, RawEvent, UpdateCx,
};

pub struct Container {
    child: Pod,
    padding_top: f64,
    padding_right: f64,
    padding_bottom: f64,
    padding_left: f64,
}

impl Container {
    pub fn new(child: impl Widget + 'static) -> Self {
        Container {
            child: Pod::new(child),
            padding_top: 0.0,
            padding_right: 0.0,
            padding_bottom: 0.0,
            padding_left: 0.0,
        }
    }

    pub fn child_mut(&mut self) -> &mut Pod {
        &mut self.child
    }

    pub fn padding(
        &mut self,
        padding_top: f64,
        padding_right: f64,
        padding_bottom: f64,
        padding_left: f64,
    ) {
        self.padding_top = padding_top;
        self.padding_right = padding_right;
        self.padding_bottom = padding_bottom;
        self.padding_left = padding_left;
    }
}

impl Widget for Container {
    fn event(&mut self, cx: &mut EventCx, event: &RawEvent) {
        self.child.event(cx, event);
    }

    fn lifecycle(&mut self, cx: &mut LifeCycleCx, event: &LifeCycle) {
        self.child.lifecycle(cx, event);
    }

    fn update(&mut self, cx: &mut UpdateCx) {
        self.child.update(cx);
    }

    fn measure(&mut self, cx: &mut LayoutCx) -> (Size, Size) {
        let _ = self.child.measure(cx);
        (Size::ZERO, Size::new(1e9, 1e9))
    }

    fn layout(&mut self, cx: &mut LayoutCx, proposed_size: Size) -> Size {
        let child_proposed = Size::new(
            proposed_size.width - self.padding_left - self.padding_right,
            proposed_size.height - self.padding_top - self.padding_bottom,
        );
        self.child.state.origin = Point::new(self.padding_left, self.padding_top);
        let child_size = self.child.layout(cx, child_proposed);
        Size::new(
            child_size.width + self.padding_left + self.padding_right,
            child_size.height + self.padding_top + self.padding_bottom,
        )
    }

    fn paint(&mut self, cx: &mut PaintCx) {
        cx.with_save(|cx| {
            let size = cx.size();
            let rect = size.to_rect();
            cx.clip(rect.clone());
            // cx.stroke_styled(
            //     rect,
            //     &Color::FUCHSIA,
            //     1.0,
            //     &StrokeStyle::new()
            //         .dash_pattern(&[4.0, 2.0])
            //         .dash_offset(8.0),
            // );
            self.child.paint(cx);
        });
    }
}
