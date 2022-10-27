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

pub mod align;
pub mod button;
pub mod container;
mod contexts;
mod core;
pub mod diagram;
pub mod hstack;
pub mod layout_observer;
pub mod list;
pub mod option;
pub mod option2;
pub mod option3;
pub mod optional;
mod raw_event;
pub mod scroll_view;
pub mod text;
pub mod vstack;

use std::any::Any;
use std::ops::{Deref, DerefMut};

use druid_shell::kurbo::{Rect, Size};

use self::contexts::LifeCycleCx;
pub use self::contexts::{AlignCx, CxState, EventCx, LayoutCx, PaintCx, PreparePaintCx, UpdateCx};
pub use self::core::Pod;
pub(crate) use self::core::{PodFlags, WidgetState};
pub use self::raw_event::{LifeCycle, RawEvent};

use self::align::SingleAlignment;

/// A basic widget trait.
pub trait Widget {
    fn event(&mut self, cx: &mut EventCx, event: &RawEvent);

    /// Propagate a lifecycle event.
    ///
    /// I am not convinced this needs to be distinct from `event`. For the
    /// moment, we're following existing Druid.
    fn lifecycle(&mut self, cx: &mut LifeCycleCx, event: &LifeCycle);

    fn update(&mut self, cx: &mut UpdateCx);

    /// Compute intrinsic sizes.
    ///
    /// This method will be called once on widget creation and then on
    /// REQUEST_UPDATE.
    fn measure(&mut self, cx: &mut LayoutCx) -> (Size, Size);

    /// Compute size given proposed size.
    ///
    /// The value will be memoized given the proposed size, invalidated
    /// on REQUEST_UPDATE. It can count on prelayout being completed.
    fn layout(&mut self, cx: &mut LayoutCx, proposed_size: Size) -> Size;

    /// Query for an alignment.
    ///
    /// This method can count on layout already having been completed.
    #[allow(unused)]
    fn align(&self, cx: &mut AlignCx, alignment: SingleAlignment) {}

    /// Prepare for painting.
    ///
    /// This method is currently a bit of a hack. It's similar to the one in
    /// Druid, which is for incremental repaint, but the primary purpose of
    /// this one is virtualized scrolling.
    ///
    /// The fact that `cx` is LayoutCx is just laziness, it should have its
    /// own cx. And the main methods on that cx should be for region-based
    /// invalidation.
    #[allow(unused)]
    fn prepare_paint(&mut self, cx: &mut LayoutCx, visible: Rect) {}

    fn paint(&mut self, cx: &mut PaintCx);
}

pub trait AnyWidget: Widget {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn type_name(&self) -> &'static str;
}

impl<W: Widget + 'static> AnyWidget for W {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl Widget for Box<dyn AnyWidget> {
    fn event(&mut self, cx: &mut EventCx, event: &RawEvent) {
        self.deref_mut().event(cx, event);
    }

    fn lifecycle(&mut self, cx: &mut LifeCycleCx, event: &LifeCycle) {
        self.deref_mut().lifecycle(cx, event);
    }

    fn update(&mut self, cx: &mut UpdateCx) {
        self.deref_mut().update(cx);
    }

    fn measure(&mut self, cx: &mut LayoutCx) -> (Size, Size) {
        self.deref_mut().measure(cx)
    }

    fn layout(&mut self, cx: &mut LayoutCx, proposed_size: Size) -> Size {
        self.deref_mut().layout(cx, proposed_size)
    }

    fn align(&self, cx: &mut AlignCx, alignment: SingleAlignment) {
        self.deref().align(cx, alignment);
    }

    fn prepare_paint(&mut self, cx: &mut PreparePaintCx, visible: Rect) {
        self.deref_mut().prepare_paint(cx, visible)
    }

    fn paint(&mut self, cx: &mut PaintCx) {
        self.deref_mut().paint(cx);
    }
}

pub trait WidgetVec {
    fn length(&self) -> usize;

    fn widgets_mut(&mut self) -> Vec<&mut dyn AnyWidget>;

    // fn add(&mut self, item: impl AnyWidget);
}
impl<T> WidgetVec for Vec<T>
where
    T: AnyWidget,
{
    fn length(&self) -> usize {
        self.len()
    }

    fn widgets_mut(&mut self) -> Vec<&mut dyn AnyWidget> {
        self.iter_mut()
            .map(|child| child as &mut dyn AnyWidget)
            .collect::<Vec<&mut dyn AnyWidget>>()
    }

    // fn add(&mut self, item: T) {
    //     self.push(item);
    // }
}

pub trait WidgetTuple {
    fn length(&self) -> usize;

    // Follows Panoramix; rethink to reduce allocation
    // Maybe SmallVec?
    fn widgets_mut(&mut self) -> Vec<&mut dyn AnyWidget>;
}

macro_rules! impl_widget_tuple {
    ( $n: tt; $( $WidgetType:ident),* ; $( $index:tt ),* ) => {
        impl< $( $WidgetType: AnyWidget ),* > WidgetTuple for ( $( $WidgetType, )* ) {
            fn length(&self) -> usize {
                $n
            }

            fn widgets_mut(&mut self) -> Vec<&mut dyn AnyWidget> {
                let mut v: Vec<&mut dyn AnyWidget> = Vec::with_capacity(self.length());
                $(
                v.push(&mut self.$index);
                )*
                v
            }

        }
    }
}

// impl<W0: AnyWidget, W1: AnyWidget> WidgetTuple for (W0, W1) {
//     fn length(&self) -> usize {
//         1
//     }

//     fn widgets_mut(&mut self) -> Vec<&mut dyn AnyWidget> {
//         let mut v: Vec<&mut dyn AnyWidget> = Vec::with_capacity(self.length());
//         v.push(&mut self.0);
//         v
//     }
// }

impl_widget_tuple!(1; W0; 0);
impl_widget_tuple!(2; W0, W1; 0, 1);
impl_widget_tuple!(3; W0, W1, W2; 0, 1, 2);
impl_widget_tuple!(4; W0, W1, W2, W3; 0, 1, 2, 3);
impl_widget_tuple!(5; W0, W1, W2, W3, W4; 0, 1, 2, 3, 4);
impl_widget_tuple!(6; W0, W1, W2, W3, W4, W5; 0, 1, 2, 3, 4, 5);
impl_widget_tuple!(7; W0, W1, W2, W3, W4, W5, W6; 0, 1, 2, 3, 4, 5, 6);
impl_widget_tuple!(8;
    W0, W1, W2, W3, W4, W5, W6, W7;
    0, 1, 2, 3, 4, 5, 6, 7
);
impl_widget_tuple!(9;
    W0, W1, W2, W3, W4, W5, W6, W7, W8;
    0, 1, 2, 3, 4, 5, 6, 7, 8
);
impl_widget_tuple!(10;
    W0, W1, W2, W3, W4, W5, W6, W7, W8, W9;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9
);
impl_widget_tuple!(11;
    W0, W1, W2, W3, W4, W5, W6, W7, W8, W9, W10;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
);
impl_widget_tuple!(12;
    W0, W1, W2, W3, W4, W5, W6, W7, W8, W9, W10, W11;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11
);
impl_widget_tuple!(13;
    W0, W1, W2, W3, W4, W5, W6, W7, W8, W9, W10, W11, W12;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12
);
impl_widget_tuple!(14;
    W0, W1, W2, W3, W4, W5, W6, W7, W8, W9, W10, W11, W12, W13;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13
);
impl_widget_tuple!(15;
    W0, W1, W2, W3, W4, W5, W6, W7, W8, W9, W10, W11, W12, W13, W14;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14
);
impl_widget_tuple!(16;
    W0, W1, W2, W3, W4, W5, W6, W7, W8, W9, W10, W11, W12, W13, W14, W15;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
);
impl_widget_tuple!(17;
    W0, W1, W2, W3, W4, W5, W6, W7, W8, W9, W10, W11, W12, W13, W14, W15, W16;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
);
impl_widget_tuple!(18;
    W0, W1, W2, W3, W4, W5, W6, W7, W8, W9, W10, W11, W12, W13, W14, W15, W16, W17;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17
);
impl_widget_tuple!(19;
    W0, W1, W2, W3, W4, W5, W6, W7, W8, W9, W10, W11, W12, W13, W14, W15, W16, W17, W18;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18
);
impl_widget_tuple!(20;
    W0, W1, W2, W3, W4, W5, W6, W7, W8, W9, W10, W11, W12, W13, W14, W15, W16, W17, W18, W19;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
);
