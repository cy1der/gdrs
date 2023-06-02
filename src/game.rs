use crate::block::Block;
use crate::constants::{
    BG_COLOR, BLOCK_COLOR, GROUND_COLOR, GROUND_COLOR_TRANSPARENT, GROUND_Y_FLIP, GROUND_Y_NORMAL,
    HEIGHT, ORB_COLOR, PLAYER_COLOR, SPIKE_COLOR, WIDTH,
};
use crate::orb::Orb;
use crate::player::Player;
use crate::spike::Spike;
use crate::surface_result::SurfaceResult;
use crate::vector::Vector;
use find_folder::Search;
use freetype::{face::LoadFlag, Bitmap, Face, GlyphSlot, Library};
use graphics::Context;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::input::{RenderArgs, UpdateArgs};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub struct Game {
    pub gl: GlGraphics,
    pub frozen: bool,
    pub attempt_count: u32,
    pub victory: bool,
    pub player: Player,
    pub blocks: Vec<Block>,
    pub spikes: Vec<Spike>,
    pub orbs: Vec<Orb>,
}

impl Default for Game {
    fn default() -> Self {
        let opengl: OpenGL = OpenGL::V4_5;

        Game {
            gl: GlGraphics::new(opengl),
            player: Player::new(),
            frozen: true,
            attempt_count: 0,
            victory: false,
            blocks: Vec::new(),
            spikes: Vec::new(),
            orbs: Vec::new(),
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            ..Default::default()
        }
    }

    pub fn render(&mut self, args: &RenderArgs, fps: i32) {
        use graphics::*;

        let player_square: [f64; 4] = rectangle::square(0.0, 0.0, self.player.size as f64);
        let ground_rect_flip: [f64; 4] =
            rectangle::rectangle_by_corners(0.0, 0.0, WIDTH as f64, GROUND_Y_FLIP as f64);
        let ground_rect: [f64; 4] = rectangle::rectangle_by_corners(
            0.0,
            GROUND_Y_NORMAL as f64,
            WIDTH as f64,
            HEIGHT as f64,
        );

        self.gl
            .draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {
                clear(BG_COLOR, gl);

                let player_transform: [[f64; 3]; 2] = c
                    .transform
                    .trans(self.player.pos.x as f64, self.player.pos.y as f64)
                    .rot_deg(if self.player.pos.x > self.player.jump.x {
                        -self.player.angle as f64
                    } else {
                        self.player.angle as f64
                    })
                    .trans(
                        -(self.player.size as f64) / 2.0,
                        -(self.player.size as f64) / 2.0,
                    );

                rectangle(PLAYER_COLOR, player_square, player_transform, gl);

                let ground_transform: [[f64; 3]; 2] = c.transform.trans(0.0, 0.0);

                rectangle(
                    if self.player.gravity_flip {
                        GROUND_COLOR_TRANSPARENT
                    } else {
                        GROUND_COLOR
                    },
                    ground_rect,
                    ground_transform,
                    gl,
                );
                rectangle(
                    if self.player.gravity_flip {
                        GROUND_COLOR
                    } else {
                        GROUND_COLOR_TRANSPARENT
                    },
                    ground_rect_flip,
                    ground_transform,
                    gl,
                );

                for block in self.blocks.iter() {
                    let block_rect: [f64; 4] = rectangle::rectangle_by_corners(
                        block.pos.x as f64,
                        block.pos.y as f64,
                        (block.pos.x + block.size.x) as f64,
                        (block.pos.y + block.size.y) as f64,
                    );

                    rectangle(BLOCK_COLOR, block_rect, c.transform, gl);
                }

                for spike in self.spikes.iter() {
                    polygon(SPIKE_COLOR, &spike.vertices, c.transform, gl);
                }

                for orb in self.orbs.iter() {
                    ellipse(
                        ORB_COLOR,
                        [
                            orb.pos.x as f64 - (orb.d as f64 / 2.0),
                            orb.pos.y as f64 - (orb.d as f64 / 2.0),
                            orb.d as f64,
                            orb.d as f64,
                        ],
                        c.transform,
                        gl,
                    );
                }

                if self.frozen {
                    if self.victory {
                        render_text([0.0, 1.0, 0.0, 1.0], "Victory ", 64, 128, 96, gl, c);
                    } else {
                        render_text(
                            [1.0, 1.0, 1.0, 1.0],
                            "Left click to jump ",
                            36,
                            if self.player.gravity_flip {
                                HEIGHT as i32 - 36
                            } else {
                                72
                            },
                            36,
                            gl,
                            c,
                        );
                        render_text(
                            [1.0, 1.0, 1.0, 1.0],
                            "Hold to keep jumping ",
                            36,
                            if self.player.gravity_flip {
                                HEIGHT as i32 - 72
                            } else {
                                112
                            },
                            36,
                            gl,
                            c,
                        );
                        render_text(
                            [1.0, 1.0, 1.0, 1.0],
                            "ESC to freeze / unfreeze ",
                            36,
                            if self.player.gravity_flip {
                                HEIGHT as i32 - 108
                            } else {
                                152
                            },
                            36,
                            gl,
                            c,
                        );
                        render_text(
                            [1.0, 1.0, 1.0, 1.0],
                            "Right click to change gravity ",
                            36,
                            if self.player.gravity_flip {
                                HEIGHT as i32 - 144
                            } else {
                                192
                            },
                            36,
                            gl,
                            c,
                        );
                        render_text(
                            [1.0, 1.0, 1.0, 1.0],
                            "R to restart ",
                            36,
                            if self.player.gravity_flip {
                                HEIGHT as i32 - 180
                            } else {
                                232
                            },
                            36,
                            gl,
                            c,
                        );
                    }
                } else if self.player.crashed {
                    render_text([1.0, 0.0, 0.0, 1.0], "Failure ", 64, 128, 96, gl, c);

                    line(
                        [1.0, 0.0, 0.0, 1.0],
                        1.5,
                        [
                            self.player.pos.x as f64 - (self.player.size as f64 / 2.0),
                            self.player.pos.y as f64 - (self.player.size as f64 / 2.0),
                            self.player.pos.x as f64 + (self.player.size as f64 / 2.0),
                            self.player.pos.y as f64 - (self.player.size as f64 / 2.0),
                        ],
                        c.transform,
                        gl,
                    );
                    line(
                        [1.0, 0.0, 0.0, 1.0],
                        1.5,
                        [
                            self.player.pos.x as f64 + (self.player.size as f64 / 2.0),
                            self.player.pos.y as f64 - (self.player.size as f64 / 2.0),
                            self.player.pos.x as f64 + (self.player.size as f64 / 2.0),
                            self.player.pos.y as f64 + (self.player.size as f64 / 2.0),
                        ],
                        c.transform,
                        gl,
                    );
                    line(
                        [1.0, 0.0, 0.0, 1.0],
                        1.5,
                        [
                            self.player.pos.x as f64 - (self.player.size as f64 / 2.0),
                            self.player.pos.y as f64 + (self.player.size as f64 / 2.0),
                            self.player.pos.x as f64 + (self.player.size as f64 / 2.0),
                            self.player.pos.y as f64 + (self.player.size as f64 / 2.0),
                        ],
                        c.transform,
                        gl,
                    );
                    line(
                        [1.0, 0.0, 0.0, 1.0],
                        1.5,
                        [
                            self.player.pos.x as f64 - (self.player.size as f64 / 2.0),
                            self.player.pos.y as f64 - (self.player.size as f64 / 2.0),
                            self.player.pos.x as f64 - (self.player.size as f64 / 2.0),
                            self.player.pos.y as f64 + (self.player.size as f64 / 2.0),
                        ],
                        c.transform,
                        gl,
                    );
                } else {
                    render_text(
                        [1.0, 1.0, 1.0, 1.0],
                        format!("Attempt {} ", self.attempt_count).as_str(),
                        36,
                        72,
                        36,
                        gl,
                        c,
                    );
                }

                render_text(
                    [1.0, 1.0, 1.0, 1.0],
                    format!("FPS: {} ", fps).as_str(),
                    36,
                    HEIGHT as i32 - 36,
                    36,
                    gl,
                    c,
                );
            });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if !self.player.crashed && !self.frozen {
            self.player.vel.y += self.player.acc.y;
            self.player.pos.y += self.player.vel.y * args.dt as f32;
            self.player.jump.x -= self.player.vel.x * args.dt as f32;

            if !self.player.grounded {
                let d: f32 = self.player.pos.dist(&self.player.jump);

                let angle: f32 = if self.player.gravity_flip {
                    -((self.player.pos.y - self.player.jump.y) / d)
                        .asin()
                        .to_degrees()
                } else {
                    ((self.player.jump.y - self.player.pos.y) / d)
                        .asin()
                        .to_degrees()
                };

                self.player.angle = angle;
            }

            let ground_check: SurfaceResult = self.player.on_ground();

            match ground_check {
                SurfaceResult::OnSurface(y) => {
                    self.player.angle = 0.0;
                    self.player.grounded = true;
                    self.player.jump = Vector::new(f32::MAX, f32::MAX);
                    self.player.pos.y = y;
                    self.player.vel.y = 0.0;
                }
                SurfaceResult::NotOnSurface => {}
            }

            let mut no_blocks: bool = self.blocks.is_empty();
            let mut no_spikes: bool = self.spikes.is_empty();
            let mut no_orbs: bool = self.orbs.is_empty();

            let mut k: usize = 0;
            while k < self.orbs.len() {
                let orb: &mut Orb = &mut self.orbs[k];
                orb.pos.x -= self.player.vel.x * args.dt as f32;

                if orb.pos.x + (orb.d / 2.0) < 0.0 {
                    self.orbs.remove(k);

                    if self.orbs.is_empty() {
                        no_orbs = true;
                        break;
                    } else {
                        continue;
                    }
                } else if self.player.check_orb_collide(orb)
                    && self.player.jumping
                    && !orb.activated
                {
                    orb.activated = true;
                    self.player.jump = Vector::new(self.player.pos.x + 250.0, self.player.pos.y);
                    self.player.grounded = false;
                    self.player.vel.y = if self.player.gravity_flip {
                        self.player.acc.y.powi(2)
                    } else {
                        -self.player.acc.y.powi(2)
                    };
                }

                k += 1;
            }

            let mut j: usize = 0;
            while j < self.spikes.len() {
                let spike: &mut Spike = &mut self.spikes[j];
                spike.pos.x -= self.player.vel.x * args.dt as f32;
                spike.vertices[0][0] = (spike.pos.x - (spike.size.x / 2.0)) as f64;
                spike.vertices[1][0] = (spike.pos.x + (spike.size.x / 2.0)) as f64;
                spike.vertices[2][0] = spike.pos.x as f64;

                if spike.pos.x + (spike.size.x / 2.0) < 0.0 {
                    self.spikes.remove(j);

                    if self.spikes.is_empty() {
                        no_spikes = true;
                        break;
                    }
                } else {
                    self.player.check_spike_crash(spike);
                }

                if self.player.crashed {
                    return;
                } else {
                    j += 1;
                }
            }

            let mut i: usize = 0;
            while i < self.blocks.len() {
                let block: &mut Block = &mut self.blocks[i];
                block.pos.x -= self.player.vel.x * args.dt as f32;

                if block.pos.x + block.size.x < 0.0 {
                    self.blocks.remove(i);

                    if self.blocks.is_empty() {
                        no_blocks = true;
                        break;
                    }
                } else {
                    let surface_check: SurfaceResult = self.player.on_block(block);

                    match surface_check {
                        SurfaceResult::OnSurface(y) => {
                            self.player.angle = 0.0;
                            self.player.grounded = true;
                            self.player.jump = Vector::new(f32::MAX, f32::MAX);
                            self.player.vel.y = 0.0;
                            self.player.pos.y = y;
                        }
                        SurfaceResult::NotOnSurface => {}
                    }

                    self.player.check_block_crash(block);
                }

                if self.player.crashed {
                    return;
                } else {
                    i += 1;
                }
            }

            if self.player.grounded && self.player.jumping {
                self.player.jump = Vector::new(self.player.pos.x + 250.0, self.player.pos.y);
                self.player.grounded = false;
                self.player.vel.y = if self.player.gravity_flip {
                    self.player.acc.y.powi(2)
                } else {
                    -self.player.acc.y.powi(2)
                };
            } else {
                self.player.grounded = false;
            }

            self.victory = no_blocks && no_spikes && no_orbs;
            self.frozen = no_blocks && no_spikes && no_orbs;
        }
    }

    pub fn initialize_level(&mut self, level_name: &str) {
        self.player = Player::new();
        self.blocks.clear();
        self.spikes.clear();
        self.orbs.clear();
        self.attempt_count += 1;
        self.frozen = true;
        self.victory = false;

        let file_path: String = format!("levels/{}", level_name);
        let path: &Path = Path::new(&file_path);
        let file = File::open(path).expect("Failed to open level contents");
        let reader: BufReader<File> = BufReader::new(file);
        let mut lines: Vec<String> = Vec::new();

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            lines.push(line);
        }

        for line in &lines {
            let fields_raw: Vec<&str> = line.split(',').collect();
            let entry_type: f32 = fields_raw[0].parse::<f32>().unwrap();

            match entry_type {
                x if x == 1.0 => {
                    let fields_nums: Vec<f32> = fields_raw[..=fields_raw.len() - 1]
                        .iter()
                        .map(|x| x.parse::<f32>().unwrap())
                        .collect();
                    self.blocks.push(Block::new(
                        Vector::new(fields_nums[1], fields_nums[2]),
                        Vector::new(fields_nums[3], fields_nums[4]),
                    ));
                }
                x if x == 2.0 => {
                    let fields_nums: Vec<f32> = fields_raw[..=fields_raw.len() - 2]
                        .iter()
                        .map(|x| x.parse::<f32>().unwrap())
                        .collect();
                    self.spikes.push(Spike::new(
                        Vector::new(fields_nums[1], fields_nums[2]),
                        Vector::new(fields_nums[3], fields_nums[4]),
                        fields_raw[5].parse::<bool>().unwrap(),
                    ))
                }
                x if x == 3.0 => {
                    let fields_nums: Vec<f32> = fields_raw[..=fields_raw.len() - 1]
                        .iter()
                        .map(|x| x.parse::<f32>().unwrap())
                        .collect();
                    self.orbs.push(Orb::new(
                        Vector::new(fields_nums[1], fields_nums[2]),
                        fields_nums[3],
                    ));
                }
                _ => {}
            }
        }
    }
}

fn glyphs(face: &mut Face, text: &str, mut x: i32, mut y: i32) -> Vec<(Texture, [f64; 2])> {
    let mut res: Vec<(Texture, [f64; 2])> = vec![];
    for ch in text.chars() {
        face.load_char(ch as usize, LoadFlag::RENDER).unwrap();
        let g: &GlyphSlot = face.glyph();

        let bitmap: Bitmap = g.bitmap();
        let texture: Texture = Texture::from_memory_alpha(
            bitmap.buffer(),
            bitmap.width() as u32,
            bitmap.rows() as u32,
            &TextureSettings::new(),
        )
        .unwrap();
        res.push((
            texture,
            [(x + g.bitmap_left()) as f64, (y - g.bitmap_top()) as f64],
        ));

        x += (g.advance().x >> 6) as i32;
        y += (g.advance().y >> 6) as i32;
    }
    res
}

fn render_text(
    color: [f32; 4],
    text: &str,
    x: i32,
    y: i32,
    text_height: u32,
    gl: &mut GlGraphics,
    c: Context,
) {
    let assets: PathBuf = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let freetype: Library = Library::init().unwrap();
    let font: PathBuf = assets.join("CaskaydiaCoveNerdFontCompleteRegular.otf");
    let mut face: Face = freetype.new_face(font, 0).unwrap();
    face.set_pixel_sizes(0, text_height).unwrap();
    let glyphs_text: &Vec<(Texture, [f64; 2])> = &glyphs(&mut face, text, x, y);

    for &(ref texture, [x, y]) in glyphs_text {
        use graphics::*;

        Image::new_color(color).draw(texture, &c.draw_state, c.transform.trans(x, y), gl);
    }
}
