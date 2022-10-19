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

use druid_shell::kurbo::{Point, Size};

use super::{
    align::SingleAlignment, contexts::LifeCycleCx, EventCx, LayoutCx, LifeCycle, PaintCx, Pod,
    RawEvent, UpdateCx, Widget,
};

pub struct HStack {
    children: Vec<Pod>,
    alignment: SingleAlignment,
    spacing: f64,
}

impl HStack {
    pub fn new(children: Vec<Pod>, alignment: SingleAlignment) -> Self {
        let spacing = 0.0;
        HStack {
            children,
            alignment,
            spacing,
        }
    }

    pub fn children_mut(&mut self) -> &mut Vec<Pod> {
        &mut self.children
    }
}

impl Widget for HStack {
    fn event(&mut self, cx: &mut EventCx, event: &RawEvent) {
        for child in &mut self.children {
            child.event(cx, event);
        }
    }

    fn lifecycle(&mut self, cx: &mut LifeCycleCx, event: &LifeCycle) {
        for child in &mut self.children {
            child.lifecycle(cx, event);
        }
    }

    fn update(&mut self, cx: &mut UpdateCx) {
        for child in &mut self.children {
            child.update(cx);
        }
    }

    fn measure(&mut self, cx: &mut LayoutCx) -> (Size, Size) {
        let mut min_size = Size::ZERO;
        let mut max_size = Size::ZERO;
        for child in &mut self.children {
            let (child_min, child_max) = child.measure(cx);
            min_size.height = min_size.height.max(child_min.height);
            min_size.width += child_min.width;
            max_size.height = max_size.height.max(child_max.height);
            max_size.width += child_max.width;
        }
        let spacing = self.spacing * (self.children.len() - 1) as f64;
        min_size.width += spacing;
        max_size.width += spacing;
        (min_size, max_size)
    }

    fn layout(&mut self, cx: &mut LayoutCx, proposed_size: Size) -> Size {
        // First, sort children in order of increasing flexibility
        let mut child_order: Vec<_> = (0..self.children.len()).collect();
        child_order.sort_by_key(|ix| self.children[*ix].width_flexibility().to_bits());
        // Offer remaining height to each child
        let mut n_remaining = self.children.len();
        let mut width_remaining = proposed_size.width - (n_remaining - 1) as f64 * self.spacing;
        for ix in child_order {
            let child_width = (width_remaining / n_remaining as f64).max(0.0);
            let child_proposed = Size::new(child_width, proposed_size.height);
            let child_size = self.children[ix].layout(cx, child_proposed);
            width_remaining -= child_size.width;
            n_remaining -= 1;
        }
        // Get alignments from children
        let alignments: Vec<f64> = self
            .children
            .iter()
            .map(|child| child.get_alignment(self.alignment))
            .collect();
        let max_align = alignments
            .iter()
            .copied()
            .reduce(f64::max)
            .unwrap_or_default();
        // Place children, using computed height and alignments
        let mut size = Size::default();
        let mut x = 0.0;
        for (i, (child, align)) in self.children.iter_mut().zip(alignments).enumerate() {
            if i != 0 {
                x += self.spacing;
            }
            let child_size = child.state.size;
            let origin = Point::new(x, max_align - align);
            child.state.origin = origin;
            size.height = size.height.max(child_size.height + origin.y);
            x += child_size.width;
        }
        size.width = x;
        size
    }

    fn align(&self, cx: &mut super::AlignCx, alignment: SingleAlignment) {
        for child in &self.children {
            child.align(cx, alignment);
        }
    }

    fn paint(&mut self, cx: &mut PaintCx) {
        for child in &mut self.children {
            child.paint(cx);
        }
    }
}
