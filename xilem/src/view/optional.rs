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

pub struct Optional<T, A, C> {
    child: C,
    show: bool,
    phantom: PhantomData<fn() -> (T, A)>,
}

pub fn optional<T, A, C>(child: C) -> Optional<T, A, C> {
    Optional::new(child)
}

impl<T, A, C> Optional<T, A, C> {
    pub fn new(child: C) -> Self {
        Optional {
            child,
            show: true,
            phantom: Default::default(),
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = show;
        self
    }
}

impl<T, A, C: View<T, A>> View<T, A> for Optional<T, A, C>
where
    C::Element: 'static,
{
    type State = (Id, C::State);

    type Element = crate::widget::optional::Optional;

    fn build(&self, cx: &mut Cx) -> (Id, Self::State, Self::Element) {
        let (id, (child_id, child_state, child_element)) =
            cx.with_new_id(|cx| self.child.build(cx));
        let mut element = crate::widget::optional::Optional::new(child_element);
        element.show(self.show);
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
            let child_changed =
                self.child
                    .rebuild(cx, &prev.child, &mut state.0, &mut state.1, child_element);
            if child_changed {
                element.child_mut().request_update();
            }

            let show_changed = child_changed || prev.show != self.show;
            if show_changed {
                element.show(self.show);
            }

            child_changed || show_changed
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

impl<T, A, C: View<T, A>> View<T, A> for Option<C>
where
    C::Element: 'static,
{
    type State = ();

    type Element = crate::widget::option::OptionWidget;

    fn build(&self, cx: &mut Cx) -> (Id, Self::State, Self::Element) {
        // let (id, (child_id, child_state, child_element)) = cx.with_new_id(|cx| {
        //     if let Some(child) = self {
        //         child.build(cx)
        //     } else {
        //         crate::widget::option::OptionWidget::new(None)
        //     }
        // });
        // let mut element = crate::widget::option::OptionWidget::new(child_element);
        // (id, (child_id, child_state), element)

        dbg!("build my option!");
        let none: Option<C::Element> = None;
        let (id, element) = cx.with_new_id(|_| crate::widget::option::OptionWidget::new(none));
        (id, (), element)
    }

    fn rebuild(
        &self,
        cx: &mut Cx,
        prev: &Self,
        id: &mut Id,
        state: &mut Self::State,
        element: &mut Self::Element,
    ) -> bool {
        dbg!("rebuild my option!");
        // cx.with_id(*id, |cx| {
        //     // let child_element = element
        //     //     .child_mut()
        //     //     .map(|child| child.downcast_mut().unwrap());
        //     // let child_changed =
        //     //     self.child
        //     //         .rebuild(cx, &prev.child, &mut state.0, &mut state.1, child_element);
        //     // if child_changed {
        //     //     element.child_mut().request_update();
        //     // }

        //     // let show_changed = child_changed || prev.show != self.show;
        //     // if show_changed {
        //     //     element.show(self.show);
        //     // }

        //     // child_changed || show_changed

        //     if let Some(child_element) = element.child_mut() {
        //         if let Some(child) = self {
        //             child.rebuild(cx, &prev.unwrap(), &mut (), &mut (), child_element);
        //         }
        //         child_element.request_update();
        //     }
        //     true
        // })
        
        // if let Some(child) = self {
        //     child.rebuild(cx, &prev.unwrap(), id, state, element)
        // } else {
        //     true
        // }
        true
    }

    fn event(
        &self,
        id_path: &[Id],
        state: &mut Self::State,
        event: Box<dyn Any>,
        app_state: &mut T,
    ) -> EventResult<A> {
        // if let Some(child) = self {
        //     let tl = &id_path[1..];
        //     child.event(tl, &mut child::State, event, app_state)
        // } else {
        //     EventResult::Stale
        // }
        EventResult::Stale
    }
}
