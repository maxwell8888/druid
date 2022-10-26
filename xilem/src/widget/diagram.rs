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

use druid_shell::{
    kurbo::{self, Affine, Circle, Ellipse, Point, Rect, Shape, Size},
    piet::{Color, PietTextLayout, RenderContext, Text, TextLayout, TextLayoutBuilder},
    text::Event,
};

use crate::view::diagram::{Connector, CustomShape, EnhancedGeometry};

use super::{
    align::{FirstBaseline, LastBaseline, SingleAlignment, VertAlignment},
    contexts::LifeCycleCx,
    AlignCx, EventCx, LayoutCx, LifeCycle, PaintCx, RawEvent, UpdateCx, Widget,
};

pub struct Diagram {
    mouse_position: Option<Point>,
    /// (start position, last position)
    down_click: Option<(Point, Point)>,

    selected_shape: Option<usize>,
    hovered_shape: Option<usize>,
    custom_shapes: Vec<CustomShape>,
    connectors: Vec<Connector>,
    // text: String,
    // color: Color,
    // layout: Option<PietTextLayout>,
    // is_wrapped: bool,
}

impl Diagram {
    pub fn new(custom_shapes: Vec<CustomShape>, connectors: Vec<Connector>) -> Diagram {
        Diagram {
            mouse_position: None,
            down_click: None,
            selected_shape: None,
            hovered_shape: None,
            // text,
            // color: Color::WHITE,
            // layout: None,
            // is_wrapped: false,
            custom_shapes,
            connectors,
        }
    }

    // pub fn set_text(&mut self, text: String) {
    //     self.text = text;
    //     self.layout = None;
    // }
}

impl Widget for Diagram {
    fn event(&mut self, cx: &mut EventCx, event: &RawEvent) {
        match event {
            RawEvent::MouseDown(mouse_event) => {
                cx.set_active(true);
                // TODO: request paint
                self.down_click = Some((mouse_event.pos, mouse_event.pos));
                self.selected_shape = None;
                for custom_shape in &mut self.custom_shapes {
                    for enhanced_geometry in &custom_shape.enhanced_geometries {
                        match enhanced_geometry {
                            // EnhancedGeometry::Circle(circle) => cx.stroke(circle, &Color::RED, 1.),
                            EnhancedGeometry::Rectangle => {
                                let rect =
                                    Rect::from_origin_size(custom_shape.origin, custom_shape.size);
                                if rect.contains(mouse_event.pos) {
                                    self.selected_shape = Some(custom_shape.id);
                                }
                            }
                            EnhancedGeometry::Ellipse => {
                                let ellipse = Ellipse::new(
                                    custom_shape.origin
                                        + Point::new(
                                            custom_shape.size.width / 2.,
                                            custom_shape.size.height / 2.,
                                        )
                                        .to_vec2(),
                                    (custom_shape.size.width / 2., custom_shape.size.height / 2.),
                                    0.,
                                );
                                if ellipse.contains(mouse_event.pos) {
                                    self.selected_shape = Some(custom_shape.id);
                                }
                            }
                        }
                    }
                }
            }
            RawEvent::MouseUp(_) => {
                if cx.is_hot() {
                    // cx.add_event(Event::new(self.id_path.clone(), ()));
                }
                self.down_click = None;
                cx.set_active(false);
                // TODO: request paint
            }
            RawEvent::MouseMove(mouse_event) => {
                dbg!(mouse_event.pos);
                if let Some((_start, last)) = &mut self.down_click {
                    if let Some(selected_shape) = self.selected_shape {
                        let shape = self
                            .custom_shapes
                            .iter_mut()
                            .find(|shape| shape.id == selected_shape)
                            .unwrap();
                        dbg!(&mouse_event.pos);
                        dbg!(&last);
                        dbg!(mouse_event.pos.to_vec2() - last.to_vec2());
                        shape.origin = shape.origin + (mouse_event.pos.to_vec2() - last.to_vec2())
                    }
                    *last = mouse_event.pos;
                }
            }
            _ => (),
        };
    }

    fn lifecycle(&mut self, cx: &mut LifeCycleCx, event: &LifeCycle) {
        match event {
            LifeCycle::HotChanged(_) => cx.request_paint(),
            _ => (),
        }
    }

    fn update(&mut self, cx: &mut UpdateCx) {
        // is mouse over diagram (this will need to get set to None when it leaves, should also probably just be using cx.is_hot())
        cx.request_layout();
    }

    fn measure(&mut self, cx: &mut LayoutCx) -> (Size, Size) {
        // let layout = cx
        //     .text()
        //     .new_text_layout(self.text.clone())
        //     .text_color(self.color.clone())
        //     .build()
        //     .unwrap();
        // let min_size = Size::ZERO;
        // let max_size = layout.size();
        // self.layout = Some(layout);
        // self.is_wrapped = false;
        // (min_size, max_size)
        (Size::new(1000., 1000.), Size::new(1000., 1000.))
    }

    fn layout(&mut self, cx: &mut LayoutCx, proposed_size: Size) -> Size {
        // let needs_wrap = proposed_size.width < cx.widget_state.max_size.width;
        // if self.is_wrapped || needs_wrap {
        //     let layout = cx
        //         .text()
        //         .new_text_layout(self.text.clone())
        //         .max_width(proposed_size.width)
        //         .text_color(self.color.clone())
        //         .build()
        //         .unwrap();
        //     let size = layout.size();
        //     self.layout = Some(layout);
        //     self.is_wrapped = needs_wrap;
        //     size
        // } else {
        //     cx.widget_state.max_size
        // }
        Size::new(1000., 1000.)
    }

    // fn align(&self, cx: &mut AlignCx, alignment: SingleAlignment) {
    //     if alignment.id() == FirstBaseline.id() {
    //         if let Some(metric) = self.layout.as_ref().unwrap().line_metric(0) {
    //             cx.aggregate(alignment, metric.baseline);
    //         }
    //     } else if alignment.id() == LastBaseline.id() {
    //         let i = self.layout.as_ref().unwrap().line_count() - 1;
    //         if let Some(metric) = self.layout.as_ref().unwrap().line_metric(i) {
    //             cx.aggregate(alignment, metric.y_offset + metric.baseline);
    //         }
    //     }
    // }

    fn paint(&mut self, cx: &mut PaintCx) {
        // cx.draw_text(self.layout.as_ref().unwrap(), Point::ZERO);
        let rect = cx.size().to_rect();
        // cx.clip(rect);
        cx.fill(rect, &Color::WHITE);

        // paint shapes
        for custom_shape in &self.custom_shapes {
            cx.with_save(|cx: &mut PaintCx| {
                cx.transform(Affine::translate(custom_shape.origin.to_vec2()));
                for enhanced_geometry in &custom_shape.enhanced_geometries {
                    match enhanced_geometry {
                        // EnhancedGeometry::Circle(circle) => cx.stroke(circle, &Color::RED, 1.),
                        EnhancedGeometry::Rectangle => {
                            let rect = custom_shape.size.to_rect();
                            cx.stroke(rect, &Color::BLUE, 1.)
                        }
                        EnhancedGeometry::Ellipse => {
                            let ellipse = Ellipse::new(
                                Point::new(
                                    custom_shape.size.width / 2.,
                                    custom_shape.size.height / 2.,
                                ),
                                (custom_shape.size.width / 2., custom_shape.size.height / 2.),
                                0.,
                            );
                            cx.stroke(ellipse, &Color::GREEN, 1.)
                        }
                    }
                }
                let mut offset = 0.;
                for text in &custom_shape.texts {
                    let layout = cx
                        .text()
                        .new_text_layout(text.child.clone())
                        .text_color(Color::RED)
                        .build()
                        .unwrap();
                    cx.draw_text(
                        &layout,
                        (
                            custom_shape.size.width / 2.,
                            custom_shape.size.height / 2. + offset,
                        ),
                    );
                    offset += layout.size().height;
                }
            });
        }
    }
}
