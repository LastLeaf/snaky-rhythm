use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Instant, Duration};
use rand;
use rand::Rng;
use glayout;
use glayout::{canvas};
use glayout::tree::{TreeNodeRc};
use glayout::canvas::element::{Element, Empty, Text, Image};
use glayout::canvas::element::style::{PositionType, DisplayType};
use super::{play_audio, get_audio_current_time};

const MAP_C: i32 = 12;
const MAP_R: i32 = 8;
const MAP_CELL_SIZE: i32 = 60;
const MAP_BORDER_W: i32 = 20;
const MAP_BORDER_H: i32 = 20;
const SCREEN_W: i32 = 1280;
const SCREEN_H: i32 = 720;
const HINT_AREA_H: i32 = 100;
const KEY_TIME_BEFORE: f32 = 0.2;
const KEY_TIME_AFTER: f32 = 0.1;

#[derive(Clone)]
pub struct LevelStates {
    pub position: (i32, i32),
    pub audio_id: i32,
    pub beats_per_min: f32,
    pub patterns: Vec<&'static str>,
}

pub struct Level {
    context: Rc<RefCell<canvas::CanvasContext>>,
    resource: super::resource::Resource,
    states: LevelStates,
}

impl Level {
    pub fn new(context: Rc<RefCell<canvas::CanvasContext>>, resource: super::resource::Resource, states: LevelStates) -> Self {
        Self {
            context,
            resource,
            states,
        }
    }
    pub fn start(self) {
        let context_clone = self.context.clone();
        let resource = self.resource.clone();

        // initial layout
        let context = context_clone.clone();
        let mut ctx = context.borrow_mut();
        let cfg = ctx.canvas_config();
        let mut root = ctx.root();

        let map_w = MAP_C * MAP_CELL_SIZE;
        let map_h = MAP_R * MAP_CELL_SIZE;
        let map_x = (SCREEN_W - MAP_BORDER_W * 2 - map_w) / 2;
        let map_y = (SCREEN_H - MAP_BORDER_H * 2 - map_h - HINT_AREA_H) / 2;
        let wrapper = element!(&cfg, Empty {
            id: "wrapper";
            Empty {
                id: "beats_hint";
                position: PositionType::Absolute;
                left: map_x;
                top: map_y + map_h + MAP_BORDER_H * 2;
                width: map_w;
                height: 40.;
                line_height: 40.;
                font_size: 30.;
                color: (0.5, 0.5, 0.5, 1.);
            };
            Text {
                id: "score";
                position: PositionType::Absolute;
                left: map_x + 660;
                top: map_y + map_h + MAP_BORDER_H * 2 + 10;
                width: map_w;
                height: 40.;
                line_height: 40.;
                font_size: 20.;
                color: (0.5, 0.5, 0.5, 1.);
                set_text("100pt");
            };
            Empty {
                id: "map_border";
                position: PositionType::Absolute;
                left: map_x - MAP_BORDER_W;
                top: map_y - MAP_BORDER_H;
                width: map_w + MAP_BORDER_W * 2;
                height: map_h + MAP_BORDER_H * 2;
                background_color: (0.5, 0.5, 0.5, 1.);
            };
            Empty {
                id: "map";
                position: PositionType::Absolute;
                left: map_x;
                top: map_y;
                width: map_w;
                height: map_h;
                background_color: (0.1, 0.1, 0.1, 1.);
                Image {
                    id: "flower";
                    position: PositionType::Absolute;
                    left: 0.;
                    top: 0.;
                    width: MAP_CELL_SIZE as f64;
                    height: MAP_CELL_SIZE as f64;
                    set_loader(resource.image("flower"));
                };
                Empty {
                    id: "snake_body";
                    position: PositionType::Absolute;
                    left: 0.;
                    top: 0.;
                };
                Empty {
                    id: "snake_head";
                    position: PositionType::Absolute;
                    left: 0.;
                    top: 0.;
                    Image {
                        position: PositionType::Absolute;
                        left: 0.;
                        top: 0.;
                        width: MAP_CELL_SIZE as f64;
                        height: MAP_CELL_SIZE as f64;
                        set_loader(resource.image("snake_normal_1"));
                    };
                    Image {
                        position: PositionType::Absolute;
                        left: 0.;
                        top: 0.;
                        width: MAP_CELL_SIZE as f64;
                        height: MAP_CELL_SIZE as f64;
                        set_loader(resource.image("snake_normal_2"));
                    };
                    Image {
                        position: PositionType::Absolute;
                        left: 0.;
                        top: 0.;
                        width: MAP_CELL_SIZE as f64;
                        height: MAP_CELL_SIZE as f64;
                        set_loader(resource.image("snake_normal_3"));
                    };
                    Image {
                        position: PositionType::Absolute;
                        display: DisplayType::None;
                        left: 0.;
                        top: 0.;
                        width: MAP_CELL_SIZE as f64;
                        height: MAP_CELL_SIZE as f64;
                        set_loader(resource.image("snake_fail"));
                    };
                };
            };
        });
        root.append(wrapper);
        let mut beats_hint = ctx.node_by_id("beats_hint").unwrap();
        let score = ctx.node_by_id("score").unwrap();
        let snake_head = ctx.node_by_id("snake_head").unwrap();
        let snake_body = ctx.node_by_id("snake_body").unwrap();
        let flower = ctx.node_by_id("flower").unwrap();
        let move_node_to_pos = |node: TreeNodeRc<Element>, (c, r)| {
            node.elem().style_mut().transform_mut().reset().offset((c * MAP_CELL_SIZE) as f64, (r * MAP_CELL_SIZE) as f64);
        };

        // init beats hint
        for i in 0..16 {
            let child = element!(&cfg, Text {
                position: PositionType::Absolute;
                display: DisplayType::Block;
                left: i as f64 * 40.;
                top: 0.;
                width: 40.;
                height: 40.;
                color: if i < 8 { (0.6, 0.6, 0.6, 1.) } else { (0.2, 0.2, 0.2, 1.) };
                set_text("x");
            });
            beats_hint.append(child);
        }

        // init snake head and body
        let mut body_position = vec![];
        let move_head = move |mut from: (i32, i32), p: (i32, i32), body_position: &mut Vec<(i32, i32)>, snake_head: TreeNodeRc<Element>, snake_body: TreeNodeRc<Element>| {
            move_node_to_pos(snake_head, p);
            for i in 0..snake_body.len() {
                let next_from = body_position[i];
                body_position[i] = from;
                let child = snake_body.child(i);
                move_node_to_pos(child, from);
                from = next_from;
            }
            p
        };
        let mut head_position = move_head((0, 0), self.states.position, &mut body_position, snake_head.clone(), snake_body.clone());
        let append_body = move |body_position: &mut Vec<(i32, i32)>, p: (i32, i32), mut snake_body: TreeNodeRc<Element>| {
            body_position.push(p);
            let child = element!(&cfg, Empty {
                position: PositionType::Absolute;
                left: 0.;
                top: 0.;
                width: MAP_CELL_SIZE as f64;
                height: MAP_CELL_SIZE as f64;
                background_color: (0.396, 0.698, 0.396, 1.);
            });
            snake_body.append(child.clone());
            move_node_to_pos(child, p);
        };
        append_body(&mut body_position, head_position, snake_body.clone());
        append_body(&mut body_position, head_position, snake_body.clone());

        // generate flower
        let mut rng = rand::thread_rng();
        let mut generate_flower = move |head_position, body_position: &mut Vec<(i32, i32)>, flower: TreeNodeRc<Element>| {
            let flower_pos: (i32, i32) = loop {
                let flower_pos = ((rng.gen::<f64>() * (MAP_C as f64)).floor() as i32, (rng.gen::<f64>() * (MAP_R as f64)).floor() as i32);
                let mut legal = true;
                if head_position == flower_pos { continue };
                for p in body_position.iter() {
                    if *p == flower_pos {
                        legal = false;
                        continue;
                    };
                }
                if !legal { continue };
                break flower_pos;
            };
            move_node_to_pos(flower, flower_pos);
            flower_pos
        };
        let mut flower_pos = generate_flower(head_position, &mut body_position, flower.clone());

        // snake eye animation
        let mut eye_frame = 0;
        let child = snake_head.child(1);
        child.elem().style_mut().display(DisplayType::None);
        let child = snake_head.child(2);
        child.elem().style_mut().display(DisplayType::None);

        // basic states
        let context = context_clone;
        let mut current_key = ctx.fetch_last_key_code();
        let mut effective_key = None;
        let mut effective_key_time = Instant::now();
        let mut failed_time = None;
        let mut direction = (1, 0);

        // beats control
        unsafe { play_audio(-1) };
        unsafe { play_audio(self.states.audio_id) };
        let step_duration = 60. / self.states.beats_per_min as f32;
        let key_time_dur = Duration::new(0, ((step_duration - 0.1) * 1000_000_000.) as u32);
        let mut prev_instant = -step_duration - (step_duration - 0.15);
        let mut beats_offset: i32 = -1;
        let mut score_num: i32 = 100;
        // let key_time_dur = Duration::new(0, ((KEY_TIME_BEFORE + KEY_TIME_AFTER) * 1000_000_000.) as u32);
        // let key_time_after_dur = Duration::new(0, (KEY_TIME_AFTER * 1000_000_000.) as u32);
        let mut can_move_time = None;
        let mut green_beat = 0;
        frame!(move |t| {
            // get current audio time
            let ts = unsafe { get_audio_current_time(self.states.audio_id) };
            if ts < prev_instant {
                prev_instant = -step_duration - (step_duration - 0.15);
                beats_offset = -1;
            }

            // handling keys
            let mut ctx = context.borrow_mut();
            let prev_key = current_key.clone();
            current_key = ctx.fetch_last_key_code();
            if current_key.is_down && current_key != prev_key {
                if effective_key.is_some() {
                    score_num -= 10;
                    score.elem().content_mut().downcast_mut::<Text>().unwrap().set_text(score_num.to_string() + "pt");
                    score.elem().style_mut().color((1.0, 0.5, 0.5, 1.0));
                }
                effective_key_time = t;
                effective_key = Some(current_key.clone());
            } else {
                if t - effective_key_time > key_time_dur {
                    effective_key_time = t;
                    if effective_key.is_some() {
                        score_num -= 10;
                        score.elem().content_mut().downcast_mut::<Text>().unwrap().set_text(score_num.to_string() + "pt");
                        score.elem().style_mut().color((1.0, 0.5, 0.5, 1.0));
                        effective_key = None;
                    }
                }
            };

            // skip unused frames
            if ts - prev_instant >= step_duration {
                prev_instant += step_duration;

                // show beats
                beats_offset += 1;
                let beats_segment = beats_offset / 8;
                let next_beats_segment = if beats_segment == self.states.patterns.len() as i32 - 1 { 0 } else { beats_segment + 1 };
                for i in 0..8 {
                    let text = self.states.patterns[beats_segment as usize].chars().nth(i).unwrap();
                    let child = beats_hint.child(i);
                    child.elem().content_mut().downcast_mut::<Text>().unwrap().set_text(text.to_string());
                    let need_highlight = beats_offset % 8 == i as i32;
                    child.elem().style_mut().color(
                        if need_highlight {
                            if text == 'x' {
                                (0.4, 1.0, 0.4, 1.)
                            } else {
                                (0.4, 1.0, 0.4, 1.)
                            }
                        } else {
                            (0.6, 0.6, 0.6, 1.)
                        }
                    );
                    if need_highlight && text == 'x' {
                        can_move_time = Some(t + key_time_dur);
                        green_beat = i as i32;
                    }
                }
                for i in 8..16 {
                    let t = self.states.patterns[next_beats_segment as usize].chars().nth(i - 8).unwrap();
                    let child = beats_hint.child(i);
                    child.elem().content_mut().downcast_mut::<Text>().unwrap().set_text(t.to_string());
                }

                // move eyes
                eye_frame = (eye_frame + 1) % 4;
                match eye_frame {
                    1 => {
                        let child = snake_head.child(0);
                        child.elem().style_mut().display(DisplayType::None);
                        let child = snake_head.child(1);
                        child.elem().style_mut().display(DisplayType::Block);
                        let child = snake_head.child(2);
                        child.elem().style_mut().display(DisplayType::None);
                    },
                    3 => {
                        let child = snake_head.child(0);
                        child.elem().style_mut().display(DisplayType::None);
                        let child = snake_head.child(1);
                        child.elem().style_mut().display(DisplayType::None);
                        let child = snake_head.child(2);
                        child.elem().style_mut().display(DisplayType::Block);
                    },
                    _ => {
                        let child = snake_head.child(0);
                        child.elem().style_mut().display(DisplayType::Block);
                        let child = snake_head.child(1);
                        child.elem().style_mut().display(DisplayType::None);
                        let child = snake_head.child(2);
                        child.elem().style_mut().display(DisplayType::None);
                    },
                }
            }

            if failed_time.is_some() {
                if t - *failed_time.as_ref().unwrap() > Duration::new(5, 0) {
                    root.remove(0);
                    let context = self.context.clone();
                    let resource = resource.clone();
                    glayout::set_timeout(move || {
                        let mut c = super::cover::Cover::new(context.clone(), resource.clone());
                        c.show();
                    }, Duration::new(0, 0));
                    return false;
                }
                return true;
            }

            if can_move_time.is_none() {
                return true;
            }
            if can_move_time.clone().unwrap() > t {
                return true;
            }
            can_move_time = None;

            // handling key action
            match effective_key {
                Some(ref key) => {
                    match key.key_code {
                        37 => {
                            direction = (-1, 0); // left
                        },
                        38 => {
                            direction = (0, -1); // up
                        },
                        39 => {
                            direction = (1, 0); // right
                        },
                        40 => {
                            direction = (0, 1); // down
                        }
                        _ => {
                            // direction = (0, 0);
                        },
                    }
                },
                None => {
                    // direction = (0, 0);
                }
            }
            effective_key = None;

            // calculate next step
            let new_head_position = (head_position.0 + direction.0, head_position.1 + direction.1);
            let body_in_new_pos = {
                let mut ret = false;
                for i in 0..(body_position.len() - 1) {
                    let p = body_position[i];
                    if p == new_head_position {
                        ret = true;
                        break;
                    }
                }
                ret
            };
            if body_in_new_pos || new_head_position.0 < 0 || new_head_position.0 >= MAP_C || new_head_position.1 < 0 || new_head_position.1 >= MAP_R {
                failed_time = Some(Instant::now());
                let child = snake_head.child(3);
                child.elem().style_mut().display(DisplayType::Block);
            } else {
                head_position = move_head(head_position, new_head_position, &mut body_position, snake_head.clone(), snake_body.clone());
                if flower_pos == head_position {
                    flower_pos = generate_flower(head_position, &mut body_position, flower.clone());
                    let tail_position = body_position[body_position.len() - 1];
                    append_body(&mut body_position, tail_position, snake_body.clone());
                    score_num += 100;
                    score.elem().content_mut().downcast_mut::<Text>().unwrap().set_text(score_num.to_string() + "pt");
                    score.elem().style_mut().color((0.5, 1.0, 0.5, 1.0));
                }
            }

            ctx.redraw();
            return true;
        });
    }
}
