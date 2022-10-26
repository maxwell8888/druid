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

use druid_shell::kurbo::{self, Affine, Point, Size};
use druid_shell::piet::Color;

use crate::{event::EventResult, id::Id};

use super::{Cx, View};

#[derive(Clone, PartialEq)]
pub struct DiagramText {
    pub child: String,
}
impl DiagramText {
    pub fn new(child: &str) -> DiagramText {
        DiagramText {
            child: child.to_owned(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum EnhancedGeometry {
    Ellipse,
    Rectangle,
    // Circle(kurbo::Circle),
}

#[derive(Clone, PartialEq)]
pub struct CustomShape {
    pub id: usize,
    pub origin: Point,
    pub size: Size,
    pub texts: Vec<DiagramText>,
    pub enhanced_geometries: Vec<EnhancedGeometry>,
}
impl CustomShape {
    pub fn new(
        id: usize,
        origin: Point,
        size: Size,
        texts: Vec<DiagramText>,
        enhanced_geometries: Vec<EnhancedGeometry>,
    ) -> CustomShape {
        CustomShape {
            id,
            origin,
            size,
            texts,
            enhanced_geometries,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ConnectorType {
    Straight,
    Curved,
}

#[derive(Clone, PartialEq)]
pub struct Connector {
    connector_type: ConnectorType,
    start: Point,
    end: Point,
    start_shape: usize,
    end_shape: usize,
}
impl Connector {
    pub fn new(
        connector_type: ConnectorType,
        start: Point,
        end: Point,
        start_shape: usize,
        end_shape: usize,
    ) -> Connector {
        Connector {
            connector_type,
            start,
            end,
            start_shape,
            end_shape,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum DiagramControl {
    Selection,
    Square,
    Ellipse,
    Circle,
}

#[derive(Clone, PartialEq)]
pub struct Diagram {
    selected_shape: Option<usize>,
    selected_control: DiagramControl,
    custom_shapes: Vec<CustomShape>,
    connectors: Vec<Connector>,
    // text: String,
    // color: Color, // font_size:
}
impl Diagram {
    pub fn new(custom_shapes: Vec<CustomShape>, connectors: Vec<Connector>) -> Diagram {
        Diagram {
            selected_shape: None,
            selected_control: DiagramControl::Selection,
            custom_shapes,
            connectors,
        }
    }
}

impl<T, A> View<T, A> for Diagram {
    type State = ();

    type Element = crate::widget::diagram::Diagram;

    fn build(&self, cx: &mut Cx) -> (Id, Self::State, Self::Element) {
        let (id, element) = cx.with_new_id(|_| {
            // let mut text_widget = crate::widget::diagram::Diagram::new(self.text.clone());
            let mut text_widget = crate::widget::diagram::Diagram::new(
                self.custom_shapes.clone(),
                self.connectors.clone(),
            );
            // text_widget.set_color(self.color.clone());
            // text_widget
            text_widget
        });
        (id, (), element)
    }

    fn rebuild(
        &self,
        _cx: &mut Cx,
        prev: &Self,
        _id: &mut crate::id::Id,
        _state: &mut Self::State,
        element: &mut Self::Element,
    ) -> bool {
        if prev != self {
            // element.set_text(self.text.clone());
            // element.set_color(self.color.clone());
            true
        } else {
            false
        }
    }

    fn event(
        &self,
        _id_path: &[crate::id::Id],
        _state: &mut Self::State,
        _event: Box<dyn Any>,
        _app_state: &mut T,
    ) -> EventResult<A> {
        EventResult::Stale
    }
}
