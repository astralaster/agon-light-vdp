use core::panic;
use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use std::time::{Instant};

use sdl2::keyboard::{Mod, Scancode};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, SurfaceCanvas, Texture, TextureCreator,BlendMode};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};
mod font;
use font::font::FONT_BYTES;
mod font_8x19;
use font_8x19::font_8x19::FONT_8x19_BYTES;
use chrono::{Local,DateTime,Datelike,Timelike};
mod audio;
use audio::audio::AudioChannels;
use sdl2::AudioSubsystem;
mod keymap;

struct Cursor {
    position_x: i32,
    position_y: i32,
    screen_width: i32,
    screen_height: i32,
    font_width: i32,
    font_height: i32,
    paged_mode: bool,
    paged_count: i32,
}

impl Cursor {
    fn new(screen_width: i32 , screen_height: i32, font_width: i32, font_height: i32) -> Cursor {
        Cursor {
            position_x: 0,
            position_y: 0,
            screen_width,
            screen_height,
            font_width,
            font_height,
            paged_mode: false,
            paged_count: 0,
        }
    }

    fn home(&mut self) {
        self.position_x = 0;
    }

    fn down(&mut self) {
        self.position_y += self.font_height;
        if self.paged_mode {
            self.paged_count += 1;
            if self.paged_count * self.font_height >= self.screen_height {
                self.paged_count = -2;
            }
        }
    }

    fn up(&mut self) {
        self.position_y -= self.font_height;
        if self.position_y < 0 {
            self.position_y = 0;
        }
    }

    fn left(&mut self) {
        self.position_x -= self.font_width;
        if self.position_x < 0 {
            self.position_x = 0;
        }
    }

    fn right(&mut self) {
        self.position_x += self.font_width;
        if self.position_x >= self.screen_width {
            self.home();
            self.down();
        }
    }
}

struct Sprite
{
    frames: Vec<u8>,
    current_frame: u8,
    pos_x: i16,
    pos_y: i16,
    visible: bool,
}

static COLOUR_LOOKUP: [sdl2::pixels::Color; 64] = [
	Color::RGB(0x00, 0x00, 0x00), Color::RGB(0x00, 0x00, 0x55), Color::RGB(0x00, 0x00, 0xAA), Color::RGB(0x00, 0x00, 0xFF),
	Color::RGB(0x00, 0x55, 0x00), Color::RGB(0x00, 0x55, 0x55), Color::RGB(0x00, 0x55, 0xAA), Color::RGB(0x00, 0x55, 0xFF),
	Color::RGB(0x00, 0xAA, 0x00), Color::RGB(0x00, 0xAA, 0x55), Color::RGB(0x00, 0xAA, 0xAA), Color::RGB(0x00, 0xAA, 0xFF),
	Color::RGB(0x00, 0xFF, 0x00), Color::RGB(0x00, 0xFF, 0x55), Color::RGB(0x00, 0xFF, 0xAA), Color::RGB(0x00, 0xFF, 0xFF),
	Color::RGB(0x55, 0x00, 0x00), Color::RGB(0x55, 0x00, 0x55), Color::RGB(0x55, 0x00, 0xAA), Color::RGB(0x55, 0x00, 0xFF),
	Color::RGB(0x55, 0x55, 0x00), Color::RGB(0x55, 0x55, 0x55), Color::RGB(0x55, 0x55, 0xAA), Color::RGB(0x55, 0x55, 0xFF),
	Color::RGB(0x55, 0xAA, 0x00), Color::RGB(0x55, 0xAA, 0x55), Color::RGB(0x55, 0xAA, 0xAA), Color::RGB(0x55, 0xAA, 0xFF),
	Color::RGB(0x55, 0xFF, 0x00), Color::RGB(0x55, 0xFF, 0x55), Color::RGB(0x55, 0xFF, 0xAA), Color::RGB(0x55, 0xFF, 0xFF),
	Color::RGB(0xAA, 0x00, 0x00), Color::RGB(0xAA, 0x00, 0x55), Color::RGB(0xAA, 0x00, 0xAA), Color::RGB(0xAA, 0x00, 0xFF),
	Color::RGB(0xAA, 0x55, 0x00), Color::RGB(0xAA, 0x55, 0x55), Color::RGB(0xAA, 0x55, 0xAA), Color::RGB(0xAA, 0x55, 0xFF),
	Color::RGB(0xAA, 0xAA, 0x00), Color::RGB(0xAA, 0xAA, 0x55), Color::RGB(0xAA, 0xAA, 0xAA), Color::RGB(0xAA, 0xAA, 0xFF),
	Color::RGB(0xAA, 0xFF, 0x00), Color::RGB(0xAA, 0xFF, 0x55), Color::RGB(0xAA, 0xFF, 0xAA), Color::RGB(0xAA, 0xFF, 0xFF),
	Color::RGB(0xFF, 0x00, 0x00), Color::RGB(0xFF, 0x00, 0x55), Color::RGB(0xFF, 0x00, 0xAA), Color::RGB(0xFF, 0x00, 0xFF),
	Color::RGB(0xFF, 0x55, 0x00), Color::RGB(0xFF, 0x55, 0x55), Color::RGB(0xFF, 0x55, 0xAA), Color::RGB(0xFF, 0x55, 0xFF),
	Color::RGB(0xFF, 0xAA, 0x00), Color::RGB(0xFF, 0xAA, 0x55), Color::RGB(0xFF, 0xAA, 0xAA), Color::RGB(0xFF, 0xAA, 0xFF),
	Color::RGB(0xFF, 0xFF, 0x00), Color::RGB(0xFF, 0xFF, 0x55), Color::RGB(0xFF, 0xFF, 0xAA), Color::RGB(0xFF, 0xFF, 0xFF),
];

struct VideoMode{
    colors: u8,
    screen_width: u32,
    screen_height: u32,
    refresh_rate: u8,
    palette: &'static[&'static Color],
}

static PALETTE_2: [&'static Color; 2] = [&COLOUR_LOOKUP[0x00], &COLOUR_LOOKUP[0x3F]];
static PALETTE_16: [&'static Color; 16] = [&COLOUR_LOOKUP[0x00], &COLOUR_LOOKUP[0x20], &COLOUR_LOOKUP[0x08], &COLOUR_LOOKUP[0x28], &COLOUR_LOOKUP[0x02], &COLOUR_LOOKUP[0x22], &COLOUR_LOOKUP[0x0A], &COLOUR_LOOKUP[0x2A], &COLOUR_LOOKUP[0x15], &COLOUR_LOOKUP[0x30], &COLOUR_LOOKUP[0x0C], &COLOUR_LOOKUP[0x3C], &COLOUR_LOOKUP[0x03], &COLOUR_LOOKUP[0x33], &COLOUR_LOOKUP[0x0F], &COLOUR_LOOKUP[0x3F]];
static PALETTE_64: [&'static Color; 64] = [&COLOUR_LOOKUP[0x00], &COLOUR_LOOKUP[0x20], &COLOUR_LOOKUP[0x08], &COLOUR_LOOKUP[0x28], &COLOUR_LOOKUP[0x02], &COLOUR_LOOKUP[0x22], &COLOUR_LOOKUP[0x0A], &COLOUR_LOOKUP[0x2A], &COLOUR_LOOKUP[0x15], &COLOUR_LOOKUP[0x30], &COLOUR_LOOKUP[0x0C], &COLOUR_LOOKUP[0x3C], &COLOUR_LOOKUP[0x03], &COLOUR_LOOKUP[0x33], &COLOUR_LOOKUP[0x0F], &COLOUR_LOOKUP[0x3F], &COLOUR_LOOKUP[0x01], &COLOUR_LOOKUP[0x04], &COLOUR_LOOKUP[0x05], &COLOUR_LOOKUP[0x06], &COLOUR_LOOKUP[0x07], &COLOUR_LOOKUP[0x09], &COLOUR_LOOKUP[0x0B], &COLOUR_LOOKUP[0x0D], &COLOUR_LOOKUP[0x0E], &COLOUR_LOOKUP[0x10], &COLOUR_LOOKUP[0x11], &COLOUR_LOOKUP[0x12], &COLOUR_LOOKUP[0x13], &COLOUR_LOOKUP[0x14], &COLOUR_LOOKUP[0x16], &COLOUR_LOOKUP[0x17], &COLOUR_LOOKUP[0x18], &COLOUR_LOOKUP[0x19], &COLOUR_LOOKUP[0x1A], &COLOUR_LOOKUP[0x1B], &COLOUR_LOOKUP[0x1C], &COLOUR_LOOKUP[0x1D], &COLOUR_LOOKUP[0x1E], &COLOUR_LOOKUP[0x1F], &COLOUR_LOOKUP[0x21], &COLOUR_LOOKUP[0x23], &COLOUR_LOOKUP[0x24], &COLOUR_LOOKUP[0x25], &COLOUR_LOOKUP[0x26], &COLOUR_LOOKUP[0x27], &COLOUR_LOOKUP[0x29], &COLOUR_LOOKUP[0x2B], &COLOUR_LOOKUP[0x2C], &COLOUR_LOOKUP[0x2D], &COLOUR_LOOKUP[0x2E], &COLOUR_LOOKUP[0x2F], &COLOUR_LOOKUP[0x31], &COLOUR_LOOKUP[0x32], &COLOUR_LOOKUP[0x34], &COLOUR_LOOKUP[0x35], &COLOUR_LOOKUP[0x36], &COLOUR_LOOKUP[0x37], &COLOUR_LOOKUP[0x38], &COLOUR_LOOKUP[0x39], &COLOUR_LOOKUP[0x3A], &COLOUR_LOOKUP[0x3B], &COLOUR_LOOKUP[0x3D], &COLOUR_LOOKUP[0x3E]];

static VIDEO_MODES: [VideoMode; 4] = [VideoMode{colors: 2, screen_width: 1024, screen_height: 768, refresh_rate: 60, palette: &PALETTE_2},
                                    VideoMode{colors: 16, screen_width: 512, screen_height: 384, refresh_rate: 60, palette: &PALETTE_16},
                                    VideoMode{colors: 64, screen_width: 320, screen_height: 200, refresh_rate: 75, palette: &PALETTE_64},
                                    VideoMode{colors: 16, screen_width: 640, screen_height: 480, refresh_rate: 60, palette: &PALETTE_16}];

pub struct VDP<'a> {
    cursor: Cursor,
    canvas: Canvas<Window>,
    texture: Texture<'a>,
    texture_creator: &'a TextureCreator<WindowContext>,
    tx: Sender<u8>,
    rx: Receiver<u8>,
    foreground_color: sdl2::pixels::Color,
    background_color: sdl2::pixels::Color,
    graph_color: sdl2::pixels::Color,
    cursor_active: bool,
    cursor_enabled: bool,
    cursor_last_change: Instant,
    vsync_counter: std::sync::Arc<std::sync::atomic::AtomicU32>,
    last_vsync: Instant,
    current_video_mode: &'static VideoMode,
    logical_coords: bool,
    terminal_mode: bool,
    terminal_underline: bool,
    terminal_reverse: bool,
    p1: Point,
    p2: Point,
    p3: Point,
    graph_origin: Point,
    font_data: Vec<u8>,
    audio_channels: AudioChannels,
    num_sprites: u8,
    num_sprites_shown: u8,
    current_sprite: u8,
    current_bitmap: u8,
    bitmaps: Vec<Option<Texture<'a>>>,
    sprites: Vec<Sprite>,
    scale_window: u8,
}

impl VDP<'_> {
    pub fn new(canvas: Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, scale_window: u8, tx: Sender<u8>, rx: Receiver<u8>, vsync_counter: std::sync::Arc<std::sync::atomic::AtomicU32>, audio_subsystem: AudioSubsystem) -> Result<VDP, String> {
        let mode =  &VIDEO_MODES[1];
    
        let texture = texture_creator.create_texture(None, sdl2::render::TextureAccess::Target, mode.screen_width, mode.screen_height).unwrap();
     
        Ok({
            let mut v=VDP {
            cursor: Cursor::new(mode.screen_width as i32, mode.screen_height as i32, 8, 8),
            canvas,
            texture,
            texture_creator,
            tx,
            rx,
            foreground_color: Color::RGB(255, 255, 255),
            background_color: Color::RGB(0, 0, 0),
            graph_color: Color::RGB(255, 255, 255),
            cursor_active: false,
            cursor_enabled: true,
            cursor_last_change: Instant::now(),
            vsync_counter,
            last_vsync: Instant::now(),
            current_video_mode: mode,
            font_data: FONT_BYTES.to_vec(),
            logical_coords: true,
            terminal_mode: false,
            terminal_reverse: false,
            terminal_underline: false,
            p1: Point::new(0,0),
            p2: Point::new(0,0),
            p3: Point::new(0,0),
            graph_origin: Point::new(0,0),
            audio_channels: AudioChannels::new(audio_subsystem),
            num_sprites: 0,
            num_sprites_shown: 0,
            current_sprite: 0,
            current_bitmap: 0,
            bitmaps: Vec::new(),
            sprites: Vec::new(),
            scale_window,
            };
            for _ in 0..256 {
                v.bitmaps.push(None);
            };
            for _ in 0..255 {
                v.sprites.push(Sprite{frames: Vec::new(), current_frame: 0,
                                      pos_x: 0, pos_y: 0, visible: false});
            }
            v
        }
        )        
    }
    
    pub fn start(&mut self) {
        self.change_mode(1);
        self.bootscreen();
    }
    
    pub fn run(&mut self) {
        if !self.do_comms() {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        
        if self.last_vsync.elapsed().as_micros() >  (1_000_000u32 / self.current_video_mode.refresh_rate as u32).into() {
            self.vsync_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            self.last_vsync = Instant::now();
    
    
            let result = self.canvas.copy(&self.texture, None, None);
            if result.is_err() {
                panic!("Fail!");
            }
            self.canvas.set_blend_mode(BlendMode::Blend);
            self.blink_cursor();
            self.show_sprites();
            self.canvas.present();
        }
    }

    pub fn send_key(&mut self, scancode: Scancode, keymod: Mod, down: bool) {
        let fabgl_vk = keymap::keymap::sdl_scancode_to_fbgl_virtual_key(scancode, keymod);
        let mut ascii = keymap::keymap::fabgl_virtual_key_to_ascii(&fabgl_vk);

        if keymod.contains(Mod::LCTRLMOD) || keymod.contains(Mod::RCTRLMOD) {
            ascii = ascii & 0x1F;
            if ascii == 0x0e {
                self.cursor.paged_mode = true;
            } else {
                if ascii == 0x0f {
                    self.cursor.paged_mode = false;
                }
            }
        }
        if (self.terminal_mode) {
            //TODO handle cursor keys.
            if down {
                self.tx.send(ascii);
            }
        } else {
            let mut modifiers: u8 = 0;
            if keymod.contains(Mod::LCTRLMOD) || keymod.contains(Mod::RCTRLMOD)   { modifiers |= 0b00000001; }
            if keymod.contains(Mod::LSHIFTMOD) || keymod.contains(Mod::RSHIFTMOD) {
                modifiers |= 0b00000010;
                if self.cursor.paged_mode && self.cursor.paged_count == -2 {
                    self.cursor.paged_count = -1;
                }
            }
            if keymod.contains(Mod::LALTMOD)                                      { modifiers |= 0b00000100; }
            if keymod.contains(Mod::RALTMOD)                                      { modifiers |= 0b00001000; }
            if keymod.contains(Mod::CAPSMOD)                                      { modifiers |= 0b00010000; }
            if keymod.contains(Mod::NUMMOD)                                       { modifiers |= 0b00100000; }
        // SCROLLLOCK is not supported by SDL2
            if keymod.contains(Mod::LGUIMOD) || keymod.contains(Mod::RGUIMOD)     { modifiers |= 0b10000000; }
            let mut keyboard_packet: Vec<u8> = vec![ascii, modifiers, fabgl_vk as u8, down as u8];
	    self.send_packet(0x1, keyboard_packet.len() as u8, &mut keyboard_packet);
        }
    }
}

impl VDP<'_> {
    fn change_mode(&mut self, mode: usize) {
        self.current_video_mode = &VIDEO_MODES[mode];
        self.cursor.screen_height = self.current_video_mode.screen_height as i32;
        self.cursor.screen_width = self.current_video_mode.screen_width as i32;
        self.canvas.window_mut().set_size(self.current_video_mode.screen_width * self.scale_window as u32, self.current_video_mode.screen_height * self.scale_window as u32).unwrap();
        self.texture = self.texture_creator.create_texture(None, sdl2::render::TextureAccess::Target, self.current_video_mode.screen_width, self.current_video_mode.screen_height).unwrap();
        self.cls();
        self.p1.x = 0;
        self.p1.y = 0;
        self.p2.x = 0;
        self.p2.y = 0;
        self.p3.x = 0;
        self.p3.y = 0;
        self.graph_origin.x = 0;
        self.graph_origin.y = 0;
    }
    
    fn get_points_from_font(bytes : Vec<u8>, underline: bool) -> Vec<Point>
    {
        let mut points: Vec<Point> = Vec::new();
        let mut y = 0;
        for byte in bytes.iter()
        {
            let b1 = if underline && y==(bytes.len() as i32 - 1) {0xffu8} else {*byte};
            for bit in 0..8
            {
                if b1 & (1 << bit) != 0
                {
                    points.push(Point::new(7 - bit, y));
                }
            }
            y = y + 1;
        }
        points
    }
    
    fn render_char(&mut self, ascii: u8)
    {
        //println!("Render {:#02X?}", ascii);
        if ascii >= 32 {
            let shifted_ascii = ascii - 32;
            let start = (self.cursor.font_height * (shifted_ascii as i32)) as usize;
            let end = start+self.cursor.font_height as usize;
            let mut points = Self::get_points_from_font(self.font_data[start..end].to_vec(),self.terminal_underline);
            
            for point in points.iter_mut() {
                point.x += self.cursor.position_x;
                point.y += self.cursor.position_y;
            }

            self.canvas.with_texture_canvas(&mut self.texture, |texture_canvas| {
                texture_canvas.set_draw_color(if self.terminal_reverse {self.foreground_color} else {self.background_color});
                texture_canvas.fill_rect(Rect::new(self.cursor.position_x, self.cursor.position_y, 8, self.cursor.font_height as u32));
                texture_canvas.set_draw_color(if self.terminal_reverse {self.background_color} else {self.foreground_color});
                texture_canvas.draw_points(&points[..]);
            });
        }
    }

    fn bootscreen(&mut self) {
        let boot_message = "Agon Quark VDP Version 1.03";
        for byte in boot_message.as_bytes() {
            self.render_char(*byte);
            self.cursor.right();
        }
        self.cursor.down();
        self.cursor.home();
    }

    
    fn blink_cursor(&mut self) {
        if self.cursor_last_change.elapsed().as_millis() > 500 {
            self.cursor_active = !self.cursor_active;
            self.cursor_last_change = Instant::now();
        }
        if self.cursor_active && self.cursor_enabled {
            self.canvas.set_draw_color(self.foreground_color);
            let output_size = self.canvas.output_size().unwrap();
            let scale_x = output_size.0 as f32 / self.current_video_mode.screen_width as f32;
            let scale_y = output_size.1 as f32 / self.current_video_mode.screen_height as f32;
            
            self.canvas.fill_rect(Rect::new((self.cursor.position_x as f32 * scale_x) as i32, (self.cursor.position_y as f32 * scale_y) as i32, (8f32 * scale_x) as u32, (self.cursor.font_height as f32 * scale_y) as u32));
        }
    }


    fn backspace(&mut self) {
        self.cursor.left();
        self.render_char(b' ');
    }

    
    fn cls(&mut self) {
        self.canvas.with_texture_canvas(&mut self.texture, |texture_canvas| {
            texture_canvas.set_draw_color(self.background_color);
            texture_canvas.clear();
        });
        self.num_sprites = 0;
        self.num_sprites_shown = 0;
        self.cursor.position_x = 0;
        self.cursor.position_y = 0;
        self.cursor.paged_count = 0;
    }
    
    fn clg(&mut self) {
        self.canvas.with_texture_canvas(&mut self.texture, |texture_canvas| {
            texture_canvas.set_draw_color(self.background_color);
            texture_canvas.clear();
        });
    }

    fn color(&mut self, c: u8) {
        if c < 128 {
            self.foreground_color = *self.current_video_mode.palette[c as usize % self.current_video_mode.palette.len()];
        } else {
            self.background_color = *self.current_video_mode.palette[c as usize % self.current_video_mode.palette.len()];
        }
    }

    fn gcolor(&mut self, _m: u8, c: u8) {
        self.graph_color = *self.current_video_mode.palette[c as usize % self.current_video_mode.palette.len()];
    }    

    fn scale(&self, p: Point) -> Point {
        if self.logical_coords
        {
            Point::new(p.x*self.cursor.screen_width/1280, p.y*self.cursor.screen_height/1024)
        }
        else
        {
            p
        }
    }

    fn translate(&self, p: Point) -> Point {
        if self.logical_coords
        {
            Point::new(p.x+self.graph_origin.x,
                       self.cursor.screen_height - 1 - p.y - self.graph_origin.y)
        }
        else
        {
            Point::new(p.x+self.graph_origin.x, p.y+self.graph_origin.y)
        }
    }

    // Return the x coordinates for each y coordinate of the line from point
    // top to the point bot. top.x and bot.x are included unless we have a
    // horizontal line, in which case we have only bot.x
    fn line_xcoords(top : Point, bot : Point) -> Vec<i32> {
        let mut xc = Vec::<i32>::new();
        let dy = (bot.y - top.y).abs();
        let dx = (top.x - bot.x).abs();
        if dy == 0 {
            xc.push(bot.x)
        } else {
            let mut y = top.y;
            if (dx > dy) {
                let mut t = -dx/2;
                let mut y = top.y;
                // 'horizontal line', iterate over x.
                xc.push(top.x);
                if top.x < bot.x {
                    for x in top.x..=bot.x {
                        t = t+dy;
                        if (t>0) {
                            t=t-dx;
                            if (y!=bot.y && y!=top.y) { 
                                xc.push(x);
                            }
                            y=y+1;
                        }
                    }
                } else {
                    for x in (bot.x..=top.x).rev() {
                        t = t+dy;
                        if (t>0) {
                            t=t-dx;
                            if (y!=bot.y && y!=top.y) { 
                                xc.push(x);
                            }
                            y=y+1;
                        }
                    }
                }
                xc.push(bot.x);
            } else {
                // 'vertical line', iterate over y, assume top.y < bot.y
                let mut t = -dy/2;
                let mut x = top.x;
                for y in top.y..=bot.y {
                    xc.push(x);
                    t += dx;
                    if t>0 {
                        if top.x > bot.x {
                            x-=1;
                        } else {
                            x+=1;
                        }
                        t=t-dy;
                    }
                }
            }
        }
        assert!((xc.len() as i32) == bot.y-top.y+1,"Number of x coordinates does not match y range");
        xc
    }
    
    fn plot(&mut self, mode: u8, x: i16, y: i16) {
        self.p3 = self.p2;
        self.p2 = self.p1;
        self.p1 = self.translate(self.scale(Point::new(x as i32,y as i32)));
        self.canvas.with_texture_canvas(&mut self.texture, |texture_canvas| {
            texture_canvas.set_draw_color(self.graph_color);
            match mode {
                4 => {println!("MOVETO");},
                5 => {
                    println!("LINETO");
                    texture_canvas.draw_line(self.p1,self.p2);
                },
                64..=71 => {
                    println!("PLOTDOT");
                    texture_canvas.draw_point(self.p1);
                },
                80..=87 => {
                    println!("TRIANGLE");
                    let mut ptop : Point = self.p1;
                    let mut pmid : Point = self.p2;
                    let mut pbot : Point = self.p3;
                    // Order the points from top to bottom.
                    if (ptop.y > pmid.y)
                    {
                        (ptop,pmid) = (pmid,ptop);
                    }
                    if (ptop.y > pbot.y)
                    {
                        (ptop,pbot) = (pbot,ptop);
                    }
                    if (pmid.y > pbot.y)
                    {
                        (pmid,pbot) = (pbot,pmid);
                    }
                    println!("Points are {},{}  {},{} {},{}",ptop.x,ptop.y,pmid.x,pmid.y,pbot.x,pbot.y);
                    // Trace the line from top to bottom using Bresenham algo.
                    // Also trace the lines from top via mid to bottom.
                    // Draw horizontal lines between them.
                    let xv1 = Self::line_xcoords(ptop, pbot);
                    let mut xv2 = Self::line_xcoords(ptop, pmid);
                    xv2.append((&mut Self::line_xcoords(pmid,pbot)[1..].to_vec()));
                    let mut y = ptop.y;
                    for (i,x1) in xv1.iter().enumerate() {
                        let x2 = xv2[i];
                        texture_canvas.draw_line(Point::new(*x1,y),Point::new(x2,y));
                        y += 1;
                    }
                },
                144..=151 => {
                    let mut r: f32 = 0.0;
                    if (mode < 148) {
                        r = ((self.p1.x * self.p1.x + self.p1.y * self.p1.y) as f32).sqrt();
                    } else {
                        let rx = self.p1.x - self.p2.x;
                        let ry = self.p1.y - self.p2.y;
                        r = ((rx*rx + ry*ry) as f32).sqrt();
                    }
                    println!("Circle at {},{} radius {}",self.p2.x, self.p2.y,r);
                    let pstart = Point::new(self.p2.x + (r as i32), self.p2.y);
                    let mut pold = pstart;
                    let mut pnew = pold;
                    // suboptimal implementaion of circle.
                    for i in 1..32 {
                        let angle = (i as f32) * 6.28318531 / 32.0;
                        pold = pnew;
                        pnew = Point::new(self.p2.x + ((r*angle.cos()) as i32),
                                          self.p2.y + ((r*angle.sin()) as i32));
                        texture_canvas.draw_line(pold,pnew);                    
                    }
                    texture_canvas.draw_line(pnew,pstart);
                },
                _ => {println!("Unsupported plot mode");}
            }
        });        
    }    

    fn get_screen_char(&mut self, x: i16, y: i16) -> u8 {
        let mut c: u8 = 0;
        if (x >= 0 &&
            x < (self.cursor.screen_width/self.cursor.font_width) as i16 &&
            y >= 0 &&
            y <  (self.cursor.screen_height/self.cursor.font_height) as i16) {
            self.canvas.with_texture_canvas(&mut self.texture, |texture_canvas| {
                let rect = Rect::new((x*8) as i32, (y*8) as i32, 8, 8);
                let v=texture_canvas.read_pixels(rect,PixelFormatEnum::RGB888).unwrap();
                // Synthesize the character bytes from the read pixels.
                // NOTE: we only do 8x8 chars for now!
                let mut bitmap = vec![0 as u8; 8];
                for cr in 0..8 {
                    let mut b = 0;
                    for cc in 0..8 {
                        let vx = cr*8*4 + cc*4;
                        let rgb = Color::RGB(v[vx+2],v[vx+1],v[vx+0]);
                        b<<=1;
                        if rgb == self.foreground_color {
                            b |= 1;
                        }
                    }
                    bitmap[cr] = b;
                }
                // Find the bitmap in the character data.
                for i in 0..96 {
                    let pat = &self.font_data[i*8..i*8+8];
                    if *pat == bitmap {
                        c = i as u8  + 32;
                        break;
                    }
                }
            });            
        }
        c
    }

    fn get_screen_pixel(&mut self, x: i16, y: i16) -> Color {
        let p1 = self.translate(self.scale(Point::new(x as i32,y as i32)));
        let mut rgb = Color::RGB(0,0,0);
        if (p1.x >=0 && p1.x < self.current_video_mode.screen_width as i32 &&
            p1.y >=0 && p1.y < self.current_video_mode.screen_height as i32) {
            self.canvas.with_texture_canvas(&mut self.texture, |texture_canvas| {
                let rect = Rect::new(p1.x, p1.y, 1, 1);
                let v=texture_canvas.read_pixels(rect,PixelFormatEnum::RGB888).unwrap();
                println!("Pixel data = {},{},{},{}",v[0],v[1],v[2],v[3]);
                rgb.r=v[2]; 
                rgb.g=v[1]; 
                rgb.b=v[0]; 
            });
        }
        rgb
    }

    fn send_cursor_position(&self) {
        let mut cursor_position_packet: Vec<u8> = vec![(self.cursor.position_x / self.cursor.font_width) as u8,
        (self.cursor.position_y / self.cursor.font_height) as u8];
        self.send_packet(0x02, cursor_position_packet.len() as u8, &mut cursor_position_packet);	
    }

    fn send_screen_char(&self, c : u8) {
        let mut screen_char_packet: Vec<u8> = vec![c];
        self.send_packet(0x03, screen_char_packet.len() as u8, &mut screen_char_packet);	        
    }

    fn send_screen_pixel(&self, rgb : Color) {
        let c = self.current_video_mode.palette.iter().position(|&e| *e==rgb).unwrap() as u8;
        let mut screen_pixel_packet: Vec<u8> = vec![rgb.r, rgb.g, rgb.b, c];
        self.send_packet(0x04, screen_pixel_packet.len() as u8, &mut screen_pixel_packet);	        
    }
    
    fn send_packet(&self, code: u8, len: u8, data: &mut Vec<u8>) {
        let mut output: Vec<u8> = Vec::new();
        output.push(code + 0x80 as u8); 
        output.push(len);
        output.append(data);
        for byte in output.iter() {
            self.tx.send(*byte);
        }
        println!("Send packet to MOS: {:#02X?}", output);
    }

    fn read_byte(&mut self) -> u8 {
        self.rx.recv().unwrap()
    }

    fn try_read_byte(&mut self) -> Result<u8, TryRecvError> {
        self.rx.try_recv()
    }

    fn read_word(&mut self) -> i16 {
        i16::from_le_bytes([self.rx.recv().unwrap(), self.rx.recv().unwrap()])
    } 

    fn read_long(&mut self) -> Color {
        let b = [self.rx.recv().unwrap(), self.rx.recv().unwrap(),
             self.rx.recv().unwrap(), self.rx.recv().unwrap()];
        Color::RGBA(b[0],b[1],b[2],b[3])
    } 

    /// @return true if data was received
    fn do_comms(&mut self) -> bool {
        if self.cursor.paged_mode {
            if self.cursor.paged_count == -2 {
                return false; // do not process any bytes while waiting for shift key.
                
            }
            if self.cursor.paged_count == -1 {
                self.check_scrolling_needed(); // do the scroll that was postponed.
                self.cursor.paged_count = 0;
            }
        }
        match self.try_read_byte() {
            Ok(n) => {
                if self.terminal_mode {
                    self.print_terminal(n);
                } else {    
                    match n {
                        n if n >= 0x20 && n != 0x7F => {
                            println!("Received character: {}", n as char);
                            self.render_char(n);
                            self.cursor.right();
                            self.check_scrolling_needed();
                        },
                        0x08 => {println!("Cursor left."); self.cursor.left();},
                        0x09 => {println!("Cursor right."); self.cursor.right();},
                        0x0A => {
                            println!("Cursor down.");
                            self.cursor.down();
                            self.check_scrolling_needed();
                        },
                        0x0B => {println!("Cursor up."); self.cursor.up();},
                        0x0C => {
                            println!("CLS.");
                            self.cls();
                        },
                        0x0D => {println!("Cursor home."); self.cursor.home();},
                        0x0E => {self.cursor.paged_mode = true; println!("PageMode ON");},
                        0x0F => {self.cursor.paged_mode = false; println!("PageMode OFF");},
                        0x10 => {
                            println!("CLG");
                            self.clg();
                        },
                        0x11 => {
                            let c = self.read_byte();
                            println!("COLOUR {}",c);
                            self.color(c);
                            
                        },
                        0x12 => {
                            let m = self.read_byte();
                            let c = self.read_byte();
                            println!("GCOL {},{}",m,c);
                            self.gcolor(m,c);
                        },
                        0x13 => {
                            let l = self.read_byte();
                            let p = self.read_byte();
                            let r = self.read_byte();
                            let g = self.read_byte();
                            let b = self.read_byte();
                            println!("Define Logical Colour?: l:{} p:{} r:{} g:{} b:{}", l, p, r, g, b);
                        },
                        0x16 => {
                            println!("MODE.");
                            let mode = self.read_byte();
                            if mode >= VIDEO_MODES.len() as u8 {
                                println!("Invalid mode: {}", mode);
                            } else {
                                self.change_mode(mode.into());
                            }
                            self.send_mode_information();
                        },
                        0x17 => {
                            println!("VDU23.");
                            match self.read_byte() {
                                0x00 => {
                                    println!("Video System Control.");
                                    self.video_system_control();
                                },
                                0x01 => {
                                    let b = self.read_byte();
                                    self.cursor_enabled = (b!=0);
                                    println!("Cursor Enable : P{}\n",self.cursor_enabled);
                                },
                                0x07 =>  {
                                    let extent = self.read_byte();
                                    let d = self.read_byte();
                                    let m = self.read_byte();
                                    println!("Scroll: full {} dir {} movement {}",extent,d,m);
                                    self.scroll(extent!=0, d, m);    
                                },
                                0x1B => {
                                    println!("Sprite Control");
                                    self.do_sprites();
                                },
                                n if n>=32 => {
                                    for i in 0..8 {
                                        let b =  self.read_byte();
                                        self.font_data[((n-32)as u32*8+i) as usize] = b;
                                    }
                                    println!("Redefine char bitmap: {}.", n);
                                },
                                n => { println!("Unknown VDU command: {:#02X?}.", n);}
                            }
                        },
                        0x19 => {
                            let mode = self.read_byte();
                            let x = self.read_word();
                            let y = self.read_word();
                            println!("PLOT {},{},{}",mode,x,y);
                            self.plot(mode,x,y);
                        },
                        0x1D => {
                            let x = self.read_word() as i32;
                            let y = self.read_word() as i32;
                            if x>= 0 && y>= 0 {
                                self.graph_origin=self.scale(Point::new(x,y));
                            }
                            println!("Graph origin {},{}",x,y);
                        },
                        0x1E => {println!("Home."); self.cursor.home();},
                        0x1F => {
                            let x = self.read_byte() as i32 * self.cursor.font_width;
                            let y = self.read_byte() as i32 * self.cursor.font_height;
                            println!("TAB({},{})",x,y);
                            if x < self.cursor.screen_width && y < self.cursor.screen_height
                            {
                                self.cursor.position_x = x;
                                self.cursor.position_y = y;
                            }
                        },
                        0x7F => {
                            println!("BACKSPACE.");
                            self.backspace();
                        },
                        n => println!("Unknown Command {:#02X?} received!", n),
                    }
                }
                true
            },
            Err(TryRecvError::Empty) => false,
            Err(TryRecvError::Disconnected) => false
        }
    }

    fn video_system_control(&mut self) {
        match self.read_byte() {
            0x80 => {
                println!("VDP_GP.");
                self.general_poll();
            },
            0x81 => {
                println!("Set keyboard layout");
                self.read_byte();
            },
            0x82 => {
                println!("Send Cursor Position");
                self.send_cursor_position();
            },
            0x83 => {
                let x = self.read_word();
                let y = self.read_word();
                let c = self.get_screen_char(x,y);
                println!("Get screen char at {},{} = {}",x,y,c);
                self.send_screen_char(c);
            },
            0x84 => {
                let x = self.read_word();
                let y = self.read_word();
                let rgb = self.get_screen_pixel(x,y);
                println!("Get screen pixel at {},{}",x,y);
                self.send_screen_pixel(rgb);
            },
            0x85 => {
                println!("VDP_AUDIO");
                self.audio();
            },
            0x86 => {
                println!("Mode Information");
                self.send_mode_information();
            },
            0x87 => {
                let m = self.read_byte();
                if m==0 {
                    self.send_time();
                } else {
                    // Set RTC not implemented.
                    for _ in 0..6 {
                        self.read_byte(); // just consume the parameters.
                    }
                }
            },
            0x88 => {
                println!("Keyboard State");
                self.keyboard_state();
            },
            0xC0 => {
                let b = self.read_byte();
                self.logical_coords = (b!=0);
                println!("Set logical coords {}\n",self.logical_coords);
            },
            0xff => {
                println!("Switch to terminal mode\n");
                self.switch_terminal_mode();
            }
            n => println!("Unknown VSC command: {:#02X?}.", n),
        }
    }

    fn scroll(&mut self, fullscreen: bool, direction: u8, delta: u8) {
        let mut xsrc : i32 = 0;
        let mut xdst : i32 = 0;
        let mut ysrc : i32 = 0;
        let mut ydst : i32 = 0;
        let mut xsize : u32 = 0;
        let mut ysize: u32 = 0;
        xsize = self.current_video_mode.screen_width;
        ysize = self.current_video_mode.screen_height;
        match direction {
            0 => { // right
                xsize -= delta as u32;
                xdst += delta as i32;
            },
            1 => { // left
                xsize -= delta as u32;
                xsrc += delta as i32;
            },
            2 => { // down
                ysize -= delta as u32;
                ydst += delta as i32;
            },
            3 => { // up
                ysize -= delta as u32;
                ysrc += delta as i32;
            },
            _ => {}
        }
        let mut scrolled_texture = self.texture_creator.create_texture(None, sdl2::render::TextureAccess::Target, self.current_video_mode.screen_width, self.current_video_mode.screen_height).unwrap();
        self.canvas.with_texture_canvas(&mut scrolled_texture, |texture_canvas| {
            texture_canvas.set_draw_color(self.background_color);
            texture_canvas.clear();
            let rect_src = Rect::new(xsrc, ysrc, xsize, ysize);
            let rect_dst = Rect::new(xdst, ydst, xsize, ysize);
            texture_canvas.copy(&self.texture, rect_src, rect_dst);
        });        
        self.texture = scrolled_texture;
    }

    fn audio(&mut self) {
        let channel = self.read_byte();
        let waveform = self.read_byte();
        let volume = self.read_byte();
        let frequency = self.read_word();
        let duration = self.read_word();
        println!("channel:{} waveform:{} volume:{} frequency:{} duration:{}", channel, waveform, volume, frequency, duration);
        let res = self.audio_channels.start_tone(channel,waveform,volume,frequency,duration);
        let mut audio_packet: Vec<u8> = vec![channel, res as u8];
        self.send_packet(0x5, audio_packet.len() as u8, &mut audio_packet);
    }

    fn general_poll(&mut self) {
        let mut packet = Vec::new();
        packet.push(self.read_byte());
        self.send_packet(0x00, packet.len() as u8, &mut packet);
    }

    fn keyboard_state(&mut self) {
        let d = self.read_byte();
        let r = self.read_byte();
        let b = self.read_byte(); // Just consume those bytes, don't implement.
        let mut packet: Vec<u8> = vec![0, 0, 0, 0, 0];
        self.send_packet(0x08, packet.len() as u8, &mut packet);        
    }
        
    fn check_scrolling_needed(&mut self) {
        if self.cursor.paged_mode && self.cursor.paged_count == -2 {
            return;
        }
        let mut overdraw = self.cursor.position_y - self.current_video_mode.screen_height as i32 + self.cursor.font_height;
        if overdraw > 0 {
            overdraw = self.cursor.font_height; // Always scroll the entire height of the font.
            println!("Need to scroll! Overdraw: {}", overdraw);
            let mut scrolled_texture = self.texture_creator.create_texture(None, sdl2::render::TextureAccess::Target, self.current_video_mode.screen_width, self.current_video_mode.screen_height).unwrap();
            self.canvas.with_texture_canvas(&mut scrolled_texture, |texture_canvas| {
                texture_canvas.set_draw_color(self.background_color);
                texture_canvas.clear();
                let rect_src = Rect::new(0, overdraw, self.current_video_mode.screen_width, self.current_video_mode.screen_height - overdraw as u32);
                let rect_dst = Rect::new(0, 0, self.current_video_mode.screen_width, self.current_video_mode.screen_height - overdraw as u32);
                texture_canvas.copy(&self.texture, rect_src, rect_dst);
            });
            self.texture = scrolled_texture;
            self.cursor.position_y -= overdraw;
        }
    }
    
    fn send_mode_information(&mut self) {
        println!("Screen width {} Screen height {}", self.cursor.screen_width, self.cursor.screen_height);
        let mut packet: Vec<u8> = vec![
            self.cursor.screen_width.to_le_bytes()[0],
            self.cursor.screen_width.to_le_bytes()[1],
            self.cursor.screen_height.to_le_bytes()[0],
            self.cursor.screen_height.to_le_bytes()[1],
            (self.cursor.screen_width / self.cursor.font_width) as u8,
            (self.cursor.screen_height / self.cursor.font_height) as u8,
            self.current_video_mode.colors,
         ];
        self.send_packet(0x06, packet.len() as u8, &mut packet);
    }

    fn send_time(&mut self) {
        let now: DateTime<Local> = Local::now();
        println!("Read RTC: {}",now);
        let yr = now.year(); // year
        let mo = now.month(); // month 1..12
        let d = now.day(); // day 1..31
        let wd = now.weekday().num_days_from_sunday(); // day of week 0=Sun .. 6=Sat
        let hr = now.hour(); // Hour
        let mi = now.minute();  // Minute
        let s = now.second();   // Second
        let mut packet: Vec<u8> = vec![(yr-1980) as u8, (mo-1) as u8, d as u8,
                                       0, wd as u8,
                                       hr as u8, mi as u8, s as u8];
        self.send_packet(0x07, packet.len() as u8, &mut packet);        

    }

    fn do_sprites(&mut self) {
        let cmd = self.read_byte();
        match cmd {
            0 => {
                let b = self.read_byte();
                println!("Select bitmap {b}");
                self.current_bitmap = b;
            },
            1 => {
                let w = self.read_word() as i32;
                let h = self.read_word() as i32;
                println!("Read bitmap {} w={} h={}", self.current_bitmap,w,h);
                if w > 0 && h > 0 {
                    let mut tex = self.texture_creator.create_texture(PixelFormatEnum::RGBA8888, sdl2::render::TextureAccess::Static,w as u32,h  as u32).unwrap();
                    let mut pixel_data = Vec::new();
                    let bitmap_size = h*w;
                    for _i in 0..bitmap_size {
                        let c1 = self.read_long();
                        let c= self.color_quantize(c1);
                        pixel_data.push(c.a);
                        pixel_data.push(c.b);
                        pixel_data.push(c.g);
                        pixel_data.push(c.r);
                    }
                    tex.update(None, &pixel_data, w as usize * 4).unwrap();
                    tex.set_blend_mode(BlendMode::Blend);
                    self.bitmaps[self.current_bitmap as usize ] = Some(tex);
                }
            },
            2 => {
                let w = self.read_word() as i32;
                let h = self.read_word() as i32;
                println!("Read bitmap {} w={} h={} one colour", self.current_bitmap,w,h);
                if w > 0 && h > 0 {
                    let mut tex = self.texture_creator.create_texture(PixelFormatEnum::RGBA8888, sdl2::render::TextureAccess::Static,w as u32,h  as u32).unwrap();
                    let c1 = self.read_long();
                    let c=self.color_quantize(c1);
                    let bitmap_size = h*w;
                    let mut pixel_data = Vec::new();
                    for _i in 0..bitmap_size {
                        pixel_data.push(c.a);
                        pixel_data.push(c.b);
                        pixel_data.push(c.g);
                        pixel_data.push(c.r);
                    }
                    tex.update(None, &pixel_data, w as usize * 4).unwrap();
                    tex.set_blend_mode(BlendMode::Blend);
                    self.bitmaps[self.current_bitmap as usize ] = Some(tex);
                }                
            },
            3 => {
                let x=self.read_word();
                let y=self.read_word();
                println!("Draw bitmap {} at {},{}",self.current_bitmap,x,y);
                match &self.bitmaps[self.current_bitmap as usize] {
                    None => {println!("Undefined bitmap");},
                    Some(bm) => { 
                        let q = bm.query();
                        let sx = q.width;
                        let sy = q.height;
                        self.canvas.with_texture_canvas(&mut self.texture, |texture_canvas| {
                            texture_canvas.copy(&bm,
                                                None,
                                                Some(Rect::new(x as i32,y as i32,sx,sy)));
                        });
                    },
                }
            },
            4 => {
                let b = self.read_byte();
                println!("Select sprite {b}");
                self.current_sprite = b;
            },
            5 => {
                println!("Clear frames of sprite {}", self.current_sprite);
                self.sprites[self.current_sprite as usize].frames = Vec::new();
                self.sprites[self.current_sprite as usize].current_frame = 0;
                self.sprites[self.current_sprite as usize].visible = false;
            },
            6 => {
                let n = self.read_byte();
                println!("Add bitmap {} as frame to sprite {}",n,self.current_sprite);
                match &self.bitmaps[n as usize] {
                    None => {println!("No bitmap defined!");},
                    Some(_) => {self.sprites[self.current_sprite as usize].frames.push(n);} 
                }
            },
            7 => {
                let b = self.read_byte();
                println!("Make {} sprites active",b);
                self.num_sprites=b;

            },
            8 => {
                println!("Next frame on sprite {}",self.current_sprite);                
                let nf = self.sprites[self.current_sprite as usize].frames.len();
                let mut f = self.sprites[self.current_sprite as usize].current_frame as usize;
                if f==nf-1 {
                    f=0;
                } else {
                    f=f+1;
                }
                self.sprites[self.current_sprite as usize].current_frame=f as u8;    
            },
            9 => {
                println!("Previous frame on sprite {}",self.current_sprite);                
                let nf = self.sprites[self.current_sprite as usize].frames.len();
                let mut f = self.sprites[self.current_sprite as usize].current_frame as usize;
                if f==0 {
                    f=nf-1;
                } else {
                    f=f-1;
                }
                self.sprites[self.current_sprite as usize].current_frame=f as u8;    
                
            },
            10 => {
                let b = self.read_byte() as usize;
                let nf = self.sprites[self.current_sprite as usize].frames.len();
                println!("Set frame {} on sprite {}",b,self.current_sprite);
                if b<nf {
                    self.sprites[self.current_sprite as usize].current_frame=b as u8;
                } else {
                    println!("Frame out of range");
                }
            },
            11 => {
                println!("Show sprite {}",self.current_sprite);
                if (self.sprites[self.current_sprite as usize].frames.len() > 0) {
                    self.sprites[self.current_sprite as usize].visible = true;
                }
                else
                {
                    println!("Try to show a sprite with no frames");
                }                
            },
            12 => {
                println!("Hide sprite {}",self.current_sprite);
                self.sprites[self.current_sprite as usize].visible = false; 
            },
            13 => {
                let x=self.read_word();
                let y=self.read_word();
                println!("Mov sprite {} to {},{}",self.current_sprite,x,y);
                self.sprites[self.current_sprite as usize].pos_x = x;
                self.sprites[self.current_sprite as usize].pos_y = y;
            },
            14 => {
                let x=self.read_word();
                let y=self.read_word();
                println!("Mov sprite {} by {},{}",self.current_sprite,x,y);
                self.sprites[self.current_sprite as usize].pos_x += x;
                self.sprites[self.current_sprite as usize].pos_y += y;
            },
            15 => {
                // actually update num_sprites_shown.
                self.num_sprites_shown = self.num_sprites;
                println!("Refresh sprites!");
            },
            16 => {
                // Reset sprite system.
                println!("Reset sprite system");
                self.cls();
                self.clear_sprites();
                for bm in self.bitmaps.iter_mut() {
                    *bm=None;
                }
                self.current_bitmap = 0;
                self.current_sprite = 0;
            },
            _ => {println!("Unsupported Sprite Command {cmd}");}    
        }
    }

    fn clear_sprites(&mut self) {
        self.num_sprites = 0;
        self.num_sprites_shown = 0;
        for s in self.sprites.iter_mut() {
            s.frames = Vec::new();
            s.current_frame = 0;
            s.visible = false;
        }
    }

    fn show_sprites(&mut self) {
        let output_size = self.canvas.output_size().unwrap();
        let scale_x = output_size.0 as f32 / self.current_video_mode.screen_width as f32;
        let scale_y = output_size.1 as f32 / self.current_video_mode.screen_height as f32;
        let mut idx=0;
        for s in self.sprites.iter() {
            if s.visible && idx < self.num_sprites_shown {
                let bm = self.bitmaps[s.frames[s.current_frame as usize] as usize].as_ref().unwrap();
                let q = bm.query();
                let sx=q.width;
                let sy=q.height;
                self.canvas.copy(bm, None,
                                 Rect::new((s.pos_x as f32 * scale_x) as i32, (s.pos_y as f32 * scale_y) as i32, sx * scale_x as u32, sy * scale_y as u32)                                 
                );
            }
            idx+=1;
        }
    }
    
    fn color_quantize(&mut self,c: sdl2::pixels::Color) -> sdl2::pixels::Color {
        Color::RGBA((c.r/64)*85, (c.g/64)*85, (c.b/64)*85, c.a)
    }

    fn switch_terminal_mode(&mut self) {
        // Select 8x19 font on 640x480 mode.
        self.font_data = FONT_8x19_BYTES[32*19..].to_vec();
        self.cursor.font_height = 19;
        self.cursor.font_width = 8;
        self.change_mode(3); // This is different from real Agon, which supports termianl mode on top of any video mode.
        self.foreground_color=Color::RGB(170, 170, 170);
        self.tx.send(0); // CP/M waits for a byte to be returned.
        self.terminal_mode = true;
    }

    fn print_terminal(&mut self, n: u8) {
        match n {
            n if n>=0x20 && n!=0x7f => {
                println!("Terminal mode: printable char {}",n);
                self.render_char(n);
                self.cursor.right();
                self.check_scrolling_needed();
            }
            0x08 => {println!("Terminal mode: BS."); self.cursor.left();},
            // Note: TAB is handled internally by CP/M BIOS.
            0x0A => {
                println!("Terminal mode: LF.");
                self.cursor.down();
                self.check_scrolling_needed();
            },
            0x0D => {println!("Terminal mode: CR."); self.cursor.home();},
            0x1b => {println!("Terminal mode: ESC.");
                     let (cmd,params) = self.parse_control();
                     match cmd {
                         b'A' => {
                             let mut n = params[0];
                             if n==0 {
                                 n=1;
                             }
                             println!("Cursor up {}",n);
                             for _ in 0..n {
                                 self.cursor.up();
                             }
                         },
                         b'B' => {
                             let mut n = params[0];
                             if n==0 {
                                 n=1;
                             }
                             println!("Cursor down {}",n);
                             for _ in 0..n {
                                 self.cursor.down();
                                 self.check_scrolling_needed();
                             }
                         },
                         b'C' => {
                             let mut n = params[0];
                             if n==0 {
                                 n=1;
                             }
                             println!("Cursor right {}",n);
                             for _ in 0..n {
                                 self.cursor.right();
                                 self.check_scrolling_needed();
                             }
                         },
                         b'D' => {
                             let mut n = params[0];
                             if n==0 {
                                 n=1;
                             }
                             println!("Cursor left {}",n);
                             for _ in 0..n {
                                 self.cursor.left();
                             }
                         },
                         b'H' | b'f' => {
                             let mut row = params[0];
                             let mut col = 1;
                             if params.len()>=2 {
                                 col = params[1];
                             }
                             if (row==0) {
                                 row=1;
                             }
                             if (col==0) {
                                 col=1;
                             }
                             println!("Cursor position r={},c={}",row,col);
                             let x = (col-1) as i32 * self.cursor.font_width;
                             let y = (row-1) as i32 * self.cursor.font_height;
                             if x < self.cursor.screen_width && y < self.cursor.screen_height
                             {
                                 self.cursor.position_x = x;
                                 self.cursor.position_y = y;
                             }
                         },
                         b'J' => {
                             let n = params[0];
                             println!("Clear screen {}",n);
                             match n {
                                 0 => {
                                     self.clear_line(0);
                                     let px=self.cursor.position_y+self.cursor.font_height;
                                     self.clear_lines(px, self.cursor.screen_height-px);
                                 },
                                 1 => {
                                     self.clear_lines(0,self.cursor.position_y);
                                     self.clear_line(1);
                                 },
                                 2 => {
                                     self.canvas.with_texture_canvas(&mut self.texture, |texture_canvas| {
                                         texture_canvas.set_draw_color(self.background_color);
                                         texture_canvas.clear();
                                     });
                                 },
                                 _ => {},
                             }                            
                         },
                         b'K' => {
                             let n = params[0];
                             println!("Clear line {}",n);
                             self.clear_line(n);
                         },
                         b'L' => {
                             let mut n = params[0];
                             if n==0 {
                                 n=1;
                             }
                             println!("Insert lines {}",n);
                             self.insert_lines(n);
                         },
                         b'M' => {
                             let mut n = params[0];
                             if n==0 {
                                 n=1;
                             }
                             println!("Delete lines {}",n);
                             self.delete_lines(n);
                         },
                         b'm' => {
                             for attr in params.iter() {
                                 match attr {
                                     0 => { 
                                         println!("Normal");
                                         self.foreground_color=Color::RGB(170, 170, 170);
                                         self.background_color=Color::RGB(0, 0, 0);
                                         self.terminal_reverse = false;
                                         self.terminal_underline = false;
                                     },
                                     1 => {
                                         println!("Bold");
                                         self.foreground_color=Color::RGB(255, 255, 255);
                                     },
                                     4 => {
                                         println!("Underline");
                                         self.terminal_underline = true;
                                     },
                                     7 => {
                                         println!("Reverse");
                                         self.terminal_reverse = true;
                                     },
                                     30..=37 => {
                                         println!("Foreground");
                                         self.foreground_color = *self.current_video_mode.palette[(attr-30) as usize];                                         
                                     },
                                     40..=47 => {
                                         println!("Background");
                                         self.background_color = *self.current_video_mode.palette[(attr-40) as usize];                                         
                                     },
                                     _ => {println!("Unimplemented attribute code {}",attr);},
                                 }
                             }
                         },                         
                         _ => {println!("Unimplemented ESC command {}",cmd);},
                     }
            },
            _ => {println!("Unimplemented control char {}",n); },    
        }
    }

    // Clear part or all of the current text line (ESC [ K command)
    fn clear_line(&mut self, n: u8) {
        let posy = self.cursor.position_y;
        let dy = self.cursor.font_height;
        let mut posx = 0;
        let mut dx = self.cursor.screen_width;
        if n==0 {
            posx = self.cursor.position_x;
            dx = self.cursor.screen_width-self.cursor.position_x;
        } else {
            if n==1 {
                dx = self.cursor.position_x+self.cursor.font_width; 
            }
        }
        if dx > 0 {
            self.canvas.with_texture_canvas(&mut self.texture, |texture_canvas| {
                texture_canvas.set_draw_color(self.background_color);
                texture_canvas.fill_rect(Rect::new(posx, posy, dx as u32, dy as u32));
            });
        }
    }

    fn clear_lines(&mut self, start: i32, h: i32) {
        if h>0 {
           let  w=self.cursor.screen_width;
            self.canvas.with_texture_canvas(&mut self.texture, |texture_canvas| {
                texture_canvas.set_draw_color(self.background_color);
                texture_canvas.fill_rect(Rect::new(0, start, w as u32, h as u32));
            });
        }
    }

    fn delete_lines(&mut self, n: u8) {
        let start = self.cursor.position_y;
        let width = self.cursor.screen_width as u32;
        let blanks = (n as i32)*self.cursor.font_height;
        let mut scrolled = self.cursor.screen_height-start-blanks;
        if scrolled <= 0 {
            scrolled = 0;
        }
        let mut scrolled_texture = self.texture_creator.create_texture(None, sdl2::render::TextureAccess::Target, self.current_video_mode.screen_width, self.current_video_mode.screen_height).unwrap();
        self.canvas.with_texture_canvas(&mut scrolled_texture, |texture_canvas| {
            let rect_unchanged = Rect::new(0,0,width,start as u32);
            texture_canvas.set_draw_color(self.background_color);
            texture_canvas.clear();
            texture_canvas.copy(&self.texture, rect_unchanged, rect_unchanged);
            let rect_src = Rect::new(0, start+blanks, width, scrolled as u32);
            let rect_dst = Rect::new(0, start, width, scrolled as u32);
            texture_canvas.copy(&self.texture, rect_src, rect_dst);
        });        
        self.texture = scrolled_texture;        
    }
        

    fn insert_lines(&mut self, n: u8) {
        let start = self.cursor.position_y;
        let width = self.cursor.screen_width as u32;
        let blanks = (n as i32)*self.cursor.font_height;
        let mut scrolled = self.cursor.screen_height-start-blanks;
        if scrolled <= 0 {
            scrolled = 0;
        }
        let mut scrolled_texture = self.texture_creator.create_texture(None, sdl2::render::TextureAccess::Target, self.current_video_mode.screen_width, self.current_video_mode.screen_height).unwrap();
        self.canvas.with_texture_canvas(&mut scrolled_texture, |texture_canvas| {
            let rect_unchanged = Rect::new(0,0,width,start as u32);
            texture_canvas.set_draw_color(self.background_color);
            texture_canvas.clear();
            texture_canvas.copy(&self.texture, rect_unchanged, rect_unchanged);
            let rect_src = Rect::new(0, start, width, scrolled as u32);
            let rect_dst = Rect::new(0, start+blanks, width, scrolled as u32);
            texture_canvas.copy(&self.texture, rect_src, rect_dst);
        });        
        self.texture = scrolled_texture;        
    }
        
    // Parse the control codes following ESC
    fn parse_control(&mut self) -> (u8, Vec<u8>) {
        let mut v = Vec::new();
        let mut p = 0;
        let mut cmd = 0;
        let c = self.read_byte();
        match c {
            b'[' => {
                loop {
                    let c = self.read_byte();
                    match c {
                        b'0'..=b'9' => { // digits are part of a parameter
                            p = p*10 + (c as i32) - 48;
                        }
                        b';' => { // params separator.
                            v.push(p as u8);
                            p = 0;
                        }
                        _ => {
                            // Any other byte terminates command.
                            v.push(p as u8);
                            cmd = c;
                            break; 
                        }                        
                    }
                };
            },
            _ => {},
        }
        (cmd,v)
    }
}
