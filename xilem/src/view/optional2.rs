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

impl<T, A, C: View<T, A>> View<T, A> for Option<C>
where
    C::Element: 'static,
{
    // type State = Option<C::State>;
    /// child id and child state
    type State = Option<(Id, C::State)>;

    // type Element = crate::widget::option::OptionWidget;
    // type Element = Option<C::Element>;
    type Element = crate::widget::option3::OptionWidget;

    fn build(&self, cx: &mut Cx) -> (Id, Self::State, Self::Element) {
        dbg!("build");
        // let (id, (child_id, child_state, child_element)) = cx.with_new_id(|cx| {
        //     if let Some(child) = self {
        //         child.build(cx)
        //     } else {
        //         crate::widget::option::OptionWidget::new(None)
        //     }
        // });
        // let mut element = crate::widget::option::OptionWidget::new(child_element);
        // (id, (child_id, child_state), element)

        // let (id, (child_id, child_state, child_element)) =
        let (id, child_stuff) = cx.with_new_id(|cx| self.as_ref().map(|child| child.build(cx)));
        // let element = crate::widget::scroll_view::ScrollView::new(child_element);
        let (id, state, child_element) =
            if let Some((child_id, child_state, child_element)) = child_stuff {
                // not sure if I need to add child Id to the state, like in scroll_view
                // (id, (child_id, child_state), element)
                (id, Some((child_id, child_state)), Some(child_element))
            } else {
                (id, None, None)
            };
        (
            id,
            state,
            crate::widget::option3::OptionWidget::new(child_element),
        )

        // dbg!("build my option!");
        // let none: Option<C::Element> = None;
        // let (id, element) = cx.with_new_id(|_| crate::widget::option::OptionWidget::new(none));
        // (id, (), element)
    }

    fn rebuild(
        &self,
        cx: &mut Cx,
        prev: &Self,
        id: &mut Id,
        state: &mut Self::State,
        element: &mut Self::Element,
    ) -> bool {
        dbg!("rebuild");
        cx.with_id(*id, |cx| {
            // let child_element = element
            //     .child_mut()
            //     .map(|child| child.downcast_mut().unwrap());
            let changed = if let Some(child_view) = self {
                if let Some(prev_child_view) = prev {
                    let (cs0, cs1) = &mut state.as_mut().unwrap();
                    // let cs1 = &mut state.as_mut().unwrap().1;
                    child_view.rebuild(
                        cx,
                        prev_child_view,
                        cs0,
                        cs1,
                        element.child_mut().unwrap().downcast_mut().unwrap(),
                    )
                } else {
                    let (child_id, child_state, child_element) = self.as_ref().unwrap().build(cx);
                    if let Some((id, state)) = state {
                        *id = child_id;
                        *state = child_state;
                    } else {
                        panic!("??")
                    }
                    let fart = Some(child_element);
                    element.set_child(fart);
                    true
                }
            } else {
                let fart: Option<<C as View<T, A>>::Element> = None;
                element.set_child(fart);
                false
            };
            if changed {
                if let Some(element_child) = element.child_mut() {
                    element_child.request_update();
                }
            }
            changed
        })

        // dbg!("rebuild my option!");

        // true

        // cx.with_id(*id, |cx| {
        //     // let child_element = element.map(|child| child.child_mut().downcast_mut().unwrap());
        //     let changed = if let Some(child) = self {
        //         if let Some(prev) = prev {
        //             child.rebuild(
        //                 cx,
        //                 prev,
        //                 id,
        //                 &mut state.as_mut().unwrap(),
        //                 &mut element.as_mut().unwrap(),
        //             )
        //         } else {
        //             child.build(cx)
        //         }
        //     } else {
        //         true
        //     };
        //     // let changed = self.map(|child| {
        //     //     child.rebuild(
        //     //         cx,
        //     //         &prev.unwrap(),
        //     //         &mut state.0,
        //     //         &mut state.1,
        //     //         child_element,
        //     //     )
        //     // });
        //     if let Some(child) = self {
        //         if changed {
        //             // element.child_mut().request_update();
        //             // child.req
        //         }
        //     }
        //     changed
        // })
        // if let Some(child) = self {
        //     element = child.build(cx);
        // }
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
        // EventResult::Stale

        // let tl = &id_path[1..];
        // if let Some(child) = self {
        //     child.event(tl, &mut state.as_mut().unwrap(), event, app_state)
        // } else {
        //     EventResult::Stale
        // }

        if let Some(child) = self {
            let tl = &id_path[1..];
            child.event(tl, &mut state.as_mut().unwrap().1, event, app_state)
        } else {
            EventResult::Stale
        }
    }
}
