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

use crate::{event::EventResult, id::Id, View};

use super::Cx;

pub struct Container<T, A, C> {
    child: C,
    padding_top: f64,
    padding_right: f64,
    padding_bottom: f64,
    padding_left: f64,
    phantom: PhantomData<fn() -> (T, A)>,
}

pub fn container<T, A, C>(child: C) -> Container<T, A, C> {
    Container::new(child)
}

impl<T, A, C> Container<T, A, C> {
    pub fn new(child: C) -> Self {
        Container {
            child,
            padding_top: 0.0,
            padding_right: 0.0,
            padding_bottom: 0.0,
            padding_left: 0.0,
            phantom: Default::default(),
        }
    }

    pub fn padding(mut self, padding: f64) -> Self {
        self.padding_top = padding;
        self.padding_right = padding;
        self.padding_bottom = padding;
        self.padding_left = padding;
        self
    }
}

impl<T, A, C: View<T, A>> View<T, A> for Container<T, A, C>
where
    C::Element: 'static,
{
    type State = (Id, C::State);

    type Element = crate::widget::container::Container;

    fn build(&self, cx: &mut Cx) -> (Id, Self::State, Self::Element) {
        let (id, (child_id, child_state, child_element)) =
            cx.with_new_id(|cx| self.child.build(cx));
        let mut element = crate::widget::container::Container::new(child_element);
        element.padding(
            self.padding_top,
            self.padding_right,
            self.padding_bottom,
            self.padding_left,
        );
        (id, (child_id, child_state), element)
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
            let child_element = element.child_mut().downcast_mut().unwrap();
            let changed =
                self.child
                    .rebuild(cx, &prev.child, &mut state.0, &mut state.1, child_element);
            if changed {
                element.child_mut().request_update();
            }
            if self.padding_top != prev.padding_top
                || self.padding_right != prev.padding_right
                || self.padding_bottom != prev.padding_bottom
                || self.padding_left != prev.padding_left
            {
                element.padding(
                    self.padding_top,
                    self.padding_right,
                    self.padding_bottom,
                    self.padding_left,
                );
            }

            changed
        })
    }

    fn event(
        &self,
        id_path: &[Id],
        state: &mut Self::State,
        event: Box<dyn Any>,
        app_state: &mut T,
    ) -> EventResult<A> {
        let tl = &id_path[1..];
        self.child.event(tl, &mut state.1, event, app_state)
    }
}
