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

use std::any::Any;

use crate::{
    event::EventResult,
    id::Id,
    view::{Cx, View},
    widget::Pod,
};

pub trait ViewSequence<T, A> {
    type State;

    type Elements;

    fn length(&self) -> usize {
        999
    }

    fn add_element(&mut self, element: T) {}

    fn build(&self, cx: &mut Cx) -> (Self::State, Vec<Pod>);

    fn rebuild(
        &self,
        cx: &mut Cx,
        prev: &Self,
        state: &mut Self::State,
        els: &mut Vec<Pod>,
    ) -> bool;

    fn event(
        &self,
        id_path: &[Id],
        state: &mut Self::State,
        event: Box<dyn Any>,
        app_state: &mut T,
    ) -> EventResult<A>;
}

macro_rules! impl_view_tuple {
    ( $n: tt; $( $t:ident),* ; $( $i:tt ),* ) => {
        impl<T, A, $( $t: View<T, A> ),* > ViewSequence<T, A> for ( $( $t, )* )
            where $( <$t as View<T, A>>::Element: 'static ),*
        {
            type State = ( $( $t::State, )* [Id; $n]);

            type Elements = ( $( $t::Element, )* );

            fn build(&self, cx: &mut Cx) -> (Self::State, Vec<Pod>) {
                let b = ( $( self.$i.build(cx), )* );
                let state = ( $( b.$i.1, )* [ $( b.$i.0 ),* ]);
                let els = vec![ $( Pod::new(b.$i.2) ),* ];
                (state, els)
            }

            fn rebuild(
                &self,
                cx: &mut Cx,
                prev: &Self,
                state: &mut Self::State,
                els: &mut Vec<Pod>,
            ) -> bool {
                let mut changed = false;
                $(
                if self.$i
                    .rebuild(cx, &prev.$i, &mut state.$n[$i], &mut state.$i,
                        els[$i].downcast_mut().unwrap())
                {
                    els[$i].request_update();
                    changed = true;
                }
                )*
                changed
            }

            fn event(
                &self,
                id_path: &[Id],
                state: &mut Self::State,
                event: Box<dyn Any>,
                app_state: &mut T,
            ) -> EventResult<A> {
                let hd = id_path[0];
                let tl = &id_path[1..];
                $(
                if hd == state.$n[$i] {
                    self.$i.event(tl, &mut state.$i, event, app_state)
                } else )* {
                    crate::event::EventResult::Stale
                }
            }
        }
    }
}

impl_view_tuple!(1; V0; 0);
// impl_view_tuple!(2; V0, V1; 0, 1);
impl<T, A, V0: View<T, A>> ViewSequence<T, A> for Vec<V0>
where
    <V0 as View<T, A>>::Element: 'static,
{
    // type State = (Vec<V0::State>, Vec<Id>);
    type State = Vec<(V0::State, Id)>;

    type Elements = Vec<V0::Element>;

    fn length(&self) -> usize {
        self.len()
    }

    fn build(&self, cx: &mut Cx) -> (Self::State, Vec<Pod>) {
        // let b = self.iter().map(|view| view.build(cx)).collect::<Vec<_>>();
        //   (self.0.build(cx), self.1.build(cx));
        // let state = (b.0 .1, b.1 .1, [b.0 .0, b.1 .0]);
        let mut states = Vec::new();
        let mut els = Vec::new();
        for view in self {
            let (id, state, element) = view.build(cx);
            states.push((state, id));
            els.push(Pod::new(element));
        }
        // b.iter()
        //     .map(|(id, state, _element)| (state, id))
        //     .collect::<Vec<_>>();
        // // let els = vec![Pod::new(b.0 .2), Pod::new(b.1 .2)];
        // let els = b
        //     .iter()
        //     .map(|(_, _, element)| {
        //         let el = element.clone();
        //         Pod::new(el)
        //     })
        //     .collect::<Vec<_>>();
        (states, els)
    }

    fn rebuild(
        &self,
        cx: &mut Cx,
        prev: &Self,
        state: &mut Self::State,
        els: &mut Vec<Pod>,
    ) -> bool {
        let mut changed = false;
        dbg!(prev.len());
        dbg!(self.len());
        dbg!(els.len());
        if self.len() > prev.len() {
            let (id, state2, element) = self.last().unwrap().build(cx);
            state.push((state2, id));
            els.push(Pod::new(element));
        }
        dbg!(prev.len());
        dbg!(self.len());
        dbg!(els.len());
        for i in 0..self.len() - 1 {
            dbg!(state.len());
            dbg!(i);
            let (st, id) = state.get_mut(i).unwrap();
            if self[i].rebuild(
                cx,
                &prev[i],
                id,
                st,
                // &mut state[i].1,
                // &mut state[i].0,
                els[i].downcast_mut().unwrap(),
            ) {
                els[i].request_update();
                changed = true;
            }
        }
        changed
    }

    fn event(
        &self,
        id_path: &[Id],
        state: &mut Self::State,
        event: Box<dyn Any>,
        app_state: &mut T,
    ) -> EventResult<A> {
        let hd = id_path[0];
        let tl = &id_path[1..];
        for i in 0..self.len() {
            if hd == state[i].1 {
                return self[0].event(tl, &mut state[i].0, event, app_state);
            }
        }
        crate::event::EventResult::Stale
        // if hd == state.2[0] {
        //     self.0.event(tl, &mut state.0, event, app_state)
        // } else if hd == state.2[1] {
        //     self.1.event(tl, &mut state.1, event, app_state)
        // } else {
        //     crate::event::EventResult::Stale
        // }
    }
}

// impl<T, A, V0: View<T, A>> ViewSequence<T, A> for [V0; 20]
// where
//     <V0 as View<T, A>>::Element: 'static,
// {
//     type State = ([V0::State; 20], [Id; 20]);

//     type Elements = [V0::Element; 20];

//     fn build(&self, cx: &mut Cx) -> (Self::State, Vec<Pod>) {
//         // let b = (self.0.build(cx), self.1.build(cx));
//         let b = [
//             self[0].build(cx),
//             self[1].build(cx),
//             self[2].build(cx),
//             self[3].build(cx),
//             self[4].build(cx),
//             self[5].build(cx),
//             self[6].build(cx),
//             self[7].build(cx),
//             self[8].build(cx),
//             self[9].build(cx),
//             self[10].build(cx),
//             self[11].build(cx),
//             self[12].build(cx),
//             self[13].build(cx),
//             self[14].build(cx),
//             self[15].build(cx),
//             self[16].build(cx),
//             self[17].build(cx),
//             self[18].build(cx),
//             self[19].build(cx),
//         ];
//         let state = (
//             [
//                 b[0].1, b[1].1, b[2].1, b[3].1, b[4].1, b[5].1, b[6].1, b[7].1, b[8].1, b[9].1,
//                 b[10].1, b[11].1, b[12].1, b[13].1, b[14].1, b[15].1, b[16].1, b[17].1, b[18].1,
//                 b[19].1,
//             ],
//             [
//                 b[0].0, b[1].0, b[2].0, b[3].0, b[4].0, b[5].0, b[6].0, b[7].0, b[8].0, b[9].0,
//                 b[10].0, b[11].0, b[12].0, b[13].0, b[14].0, b[15].0, b[16].0, b[17].0, b[18].0,
//                 b[19].0,
//             ],
//         );
//         // let els = vec![Pod::new(b.0 .2), Pod::new(b.1 .2)];
//         // let els = b
//         //     .iter()
//         //     .copied()
//         //     .map(|thing| Pod::new(thing.2))
//         //     .collect::<Vec<_>>();
//         let mut els = Vec::new();
//         for i in 0..20 {
//             els.push(Pod::new(b[i].2));
//         }
//         (state, els)
//     }

//     fn rebuild(
//         &self,
//         cx: &mut Cx,
//         prev: &Self,
//         state: &mut Self::State,
//         els: &mut Vec<Pod>,
//     ) -> bool {
//         let mut changed = false;
//         for i in 0..20 {
//             if self[i].rebuild(
//                 cx,
//                 &prev[i],
//                 &mut state.1[i],
//                 &mut state.0[i],
//                 els[i].downcast_mut().unwrap(),
//             ) {
//                 els[i].request_update();
//                 changed = true;
//             }
//         }
//         // if self.1.rebuild(
//         //     cx,
//         //     &prev.1,
//         //     &mut state.2[1],
//         //     &mut state.1,
//         //     els[1].downcast_mut().unwrap(),
//         // ) {
//         //     els[1].request_update();
//         //     changed = true;
//         // }
//         changed
//     }

//     fn event(
//         &self,
//         id_path: &[Id],
//         state: &mut Self::State,
//         event: Box<dyn Any>,
//         app_state: &mut T,
//     ) -> EventResult<A> {
//         let hd = id_path[0];
//         let tl = &id_path[1..];
//         if hd == state.1[0] {
//             self[0].event(tl, &mut state.0[0], event, app_state)
//         } else if hd == state.1[1] {
//             self[1].event(tl, &mut state.0[1], event, app_state)
//         } else if hd == state.1[2] {
//             self[2].event(tl, &mut state.0[2], event, app_state)
//         } else if hd == state.1[3] {
//             self[3].event(tl, &mut state.0[3], event, app_state)
//         } else if hd == state.1[4] {
//             self[4].event(tl, &mut state.0[4], event, app_state)
//         } else if hd == state.1[5] {
//             self[5].event(tl, &mut state.0[5], event, app_state)
//         } else if hd == state.1[6] {
//             self[6].event(tl, &mut state.0[6], event, app_state)
//         } else if hd == state.1[7] {
//             self[7].event(tl, &mut state.0[7], event, app_state)
//         } else if hd == state.1[8] {
//             self[8].event(tl, &mut state.0[8], event, app_state)
//         } else if hd == state.1[9] {
//             self[9].event(tl, &mut state.0[9], event, app_state)
//         } else if hd == state.1[10] {
//             self[10].event(tl, &mut state.0[10], event, app_state)
//         } else if hd == state.1[11] {
//             self[11].event(tl, &mut state.0[11], event, app_state)
//         } else if hd == state.1[12] {
//             self[12].event(tl, &mut state.0[12], event, app_state)
//         } else if hd == state.1[13] {
//             self[13].event(tl, &mut state.0[13], event, app_state)
//         } else if hd == state.1[14] {
//             self[14].event(tl, &mut state.0[14], event, app_state)
//         } else if hd == state.1[15] {
//             self[15].event(tl, &mut state.0[15], event, app_state)
//         } else if hd == state.1[16] {
//             self[16].event(tl, &mut state.0[16], event, app_state)
//         } else if hd == state.1[17] {
//             self[17].event(tl, &mut state.0[17], event, app_state)
//         } else if hd == state.1[18] {
//             self[18].event(tl, &mut state.0[18], event, app_state)
//         } else if hd == state.1[19] {
//             self[19].event(tl, &mut state.0[19], event, app_state)
//         } else {
//             crate::event::EventResult::Stale
//         }
//     }
// }

impl<T, A, V0: View<T, A>, V1: View<T, A>> ViewSequence<T, A> for (V0, V1)
where
    <V0 as View<T, A>>::Element: 'static,
    <V1 as View<T, A>>::Element: 'static,
{
    type State = (V0::State, V1::State, [Id; 2]);

    type Elements = (V0::Element, V1::Element);

    fn build(&self, cx: &mut Cx) -> (Self::State, Vec<Pod>) {
        let b = (self.0.build(cx), self.1.build(cx));
        let state = (b.0 .1, b.1 .1, [b.0 .0, b.1 .0]);
        let els = vec![Pod::new(b.0 .2), Pod::new(b.1 .2)];
        (state, els)
    }

    fn rebuild(
        &self,
        cx: &mut Cx,
        prev: &Self,
        state: &mut Self::State,
        els: &mut Vec<Pod>,
    ) -> bool {
        let mut changed = false;
        if self.0.rebuild(
            cx,
            &prev.0,
            &mut state.2[0],
            &mut state.0,
            els[0].downcast_mut().unwrap(),
        ) {
            els[0].request_update();
            changed = true;
        }
        if self.1.rebuild(
            cx,
            &prev.1,
            &mut state.2[1],
            &mut state.1,
            els[1].downcast_mut().unwrap(),
        ) {
            els[1].request_update();
            changed = true;
        }
        changed
    }

    fn event(
        &self,
        id_path: &[Id],
        state: &mut Self::State,
        event: Box<dyn Any>,
        app_state: &mut T,
    ) -> EventResult<A> {
        let hd = id_path[0];
        let tl = &id_path[1..];
        if hd == state.2[0] {
            self.0.event(tl, &mut state.0, event, app_state)
        } else if hd == state.2[1] {
            self.1.event(tl, &mut state.1, event, app_state)
        } else {
            crate::event::EventResult::Stale
        }
    }
}
impl_view_tuple!(3; V0, V1, V2; 0, 1, 2);
impl_view_tuple!(4; V0, V1, V2, V3; 0, 1, 2, 3);
impl_view_tuple!(5; V0, V1, V2, V3, V4; 0, 1, 2, 3, 4);
impl_view_tuple!(6; V0, V1, V2, V3, V4, V5; 0, 1, 2, 3, 4, 5);
impl_view_tuple!(7; V0, V1, V2, V3, V4, V5, V6; 0, 1, 2, 3, 4, 5, 6);
impl_view_tuple!(8;
    V0, V1, V2, V3, V4, V5, V6, V7;
    0, 1, 2, 3, 4, 5, 6, 7
);
impl_view_tuple!(9;
    V0, V1, V2, V3, V4, V5, V6, V7, V8;
    0, 1, 2, 3, 4, 5, 6, 7, 8
);
impl_view_tuple!(10;
    V0, V1, V2, V3, V4, V5, V6, V7, V8, V9;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9
);
impl_view_tuple!(11;
    V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
);
impl_view_tuple!(12;
    V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11
);
impl_view_tuple!(13;
    V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12
);
impl_view_tuple!(14;
    V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13
);
impl_view_tuple!(15;
    V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14
);
impl_view_tuple!(16;
    V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
);
impl_view_tuple!(17;
    V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
);
impl_view_tuple!(18;
    V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17
);
impl_view_tuple!(19;
    V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18
);
impl_view_tuple!(20;
    V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
);
