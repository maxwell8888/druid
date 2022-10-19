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

use std::{any::Any, marker::PhantomData};

use crate::{
    event::EventResult,
    id::Id,
    view_seq::ViewSequence,
    widget::{
        align::{Center, SingleAlignment, Top},
        WidgetTuple,
    },
    VertAlignment,
};

use super::{Cx, View};

pub struct HStack<T, A, VT: ViewSequence<T, A>> {
    children: VT,
    cross_axis_alignment: SingleAlignment,
    phantom: PhantomData<fn() -> (T, A)>,
}

pub fn h_stack<T, A, VT: ViewSequence<T, A>>(children: VT) -> HStack<T, A, VT> {
    HStack::new(children)
}

impl<T, A, VT: ViewSequence<T, A>> HStack<T, A, VT> {
    pub fn new(children: VT) -> Self {
        let phantom = Default::default();
        let cross_axis_alignment = SingleAlignment::from_vert(&Center);
        HStack {
            children,
            cross_axis_alignment,
            phantom,
        }
    }

    pub fn cross_axis_alignment(mut self, align: &impl VertAlignment) -> Self {
        self.cross_axis_alignment = SingleAlignment::from_vert(align);
        self
    }
}

impl<T, A, VT: ViewSequence<T, A>> View<T, A> for HStack<T, A, VT>
where
    VT::Elements: WidgetTuple,
{
    type State = VT::State;

    type Element = crate::widget::hstack::HStack;

    fn build(&self, cx: &mut Cx) -> (Id, Self::State, Self::Element) {
        let (id, (state, elements)) = cx.with_new_id(|cx| self.children.build(cx));
        let row = crate::widget::hstack::HStack::new(elements, self.cross_axis_alignment);
        (id, state, row)
    }

    fn rebuild(
        &self,
        cx: &mut Cx,
        prev: &Self,
        id: &mut Id,
        state: &mut Self::State,
        element: &mut Self::Element,
    ) -> bool {
        cx.with_id(*id, |cx| {
            self.children
                .rebuild(cx, &prev.children, state, element.children_mut())
        })
    }

    fn event(
        &self,
        id_path: &[Id],
        state: &mut Self::State,
        event: Box<dyn Any>,
        app_state: &mut T,
    ) -> EventResult<A> {
        self.children.event(id_path, state, event, app_state)
    }
}
