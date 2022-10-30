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
