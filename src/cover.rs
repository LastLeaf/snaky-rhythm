use std::rc::Rc;
use std::cell::RefCell;
use glayout::{canvas};
use glayout::canvas::element::{Event, Element, Empty, Text, Image};
use glayout::canvas::element::style::{PositionType, DisplayType};

pub struct Cover {
    context: Rc<RefCell<canvas::CanvasContext>>,
    resource: super::resource::Resource,
}

fn start_level(context: &Rc<RefCell<canvas::CanvasContext>>, resource: &super::resource::Resource, num: usize) {
    let ctx = context.clone();
    ctx.borrow_mut().root().remove(0);
    let level = super::level::Level::new(context.clone(), resource.clone(), super::levels::level(num));
    level.start();
}

impl Cover {
    pub fn new(context: Rc<RefCell<canvas::CanvasContext>>, resource: super::resource::Resource) -> Self {
        Self {
            context,
            resource
        }
    }
    pub fn show(&mut self) {
        let ctx_clone_1 = self.context.clone();
        let ctx_clone_2 = self.context.clone();
        let ctx_clone_3 = self.context.clone();
        let resource_clone_1 = self.resource.clone();
        let resource_clone_2 = self.resource.clone();
        let resource_clone_3 = self.resource.clone();

        let mut ctx = self.context.borrow_mut();
        let cfg = ctx.canvas_config();
        let mut root = ctx.root();

        let cover = element!(&cfg, Empty {
            Image {
                position: PositionType::Absolute;
                left: 400.;
                top: 200.;
                width: 200.;
                height: 200.;
                set_loader(self.resource.image("snake_normal_1"));
            };
            Empty {
                position: PositionType::Absolute;
                left: 400.;
                top: 440.;
                color: (0.396, 0.698, 0.396, 1.);
                Text {
                    display: DisplayType::Block;
                    font_size: 30.;
                    line_height: 50.;
                    set_text("Snaky Rhythm");
                };
            };
            Empty {
                position: PositionType::Absolute;
                left: 400.;
                top: 500.;
                width: 150.;
                height: 36.;
                color: (0.2, 0.2, 0.2, 1.);
                background_color: (0.396, 0.698, 0.396, 1.);
                Text {
                    display: DisplayType::Block;
                    font_size: 24.;
                    line_height: 36.;
                    set_text(" Level 1 >");
                    @ "touchend" => move |_: &Event| {
                        start_level(&ctx_clone_1, &resource_clone_1, 0);
                    };
                };
            };
            Empty {
                position: PositionType::Absolute;
                left: 400.;
                top: 550.;
                width: 150.;
                height: 36.;
                color: (0.2, 0.2, 0.2, 1.);
                background_color: (0.396, 0.698, 0.396, 1.);
                Text {
                    display: DisplayType::Block;
                    font_size: 24.;
                    line_height: 36.;
                    set_text(" Level 2 >");
                    @ "touchend" => move |_: &Event| {
                        start_level(&ctx_clone_2, &resource_clone_2, 1);
                    };
                };
            };
            Empty {
                position: PositionType::Absolute;
                left: 400.;
                top: 600.;
                width: 150.;
                height: 36.;
                color: (0.2, 0.2, 0.2, 1.);
                background_color: (0.396, 0.698, 0.396, 1.);
                Text {
                    display: DisplayType::Block;
                    font_size: 24.;
                    line_height: 36.;
                    set_text(" Level 3 >");
                    @ "touchend" => move |_: &Event| {
                        start_level(&ctx_clone_3, &resource_clone_3, 2);
                    };
                };
            };
            Empty {
                position: PositionType::Absolute;
                left: 400.;
                top: 680.;
                color: (0.5, 0.5, 0.5, 1.);
                Text {
                    display: DisplayType::Block;
                    font_size: 16.;
                    line_height: 24.;
                    set_text("A game by LastLeaf, for GitHub Game Off 2018");
                };
            };
        });
        root.append(cover);
    }
}
