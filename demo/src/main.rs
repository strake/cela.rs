#![feature(slice_patterns)]

extern crate bin;
extern crate cela;
#[macro_use]
extern crate null_terminated;
extern crate rand;
extern crate rsdl2;

use std as core;

use cela::*;
use rand::Rng;

const grid_size: [usize; 2] = [64; 2];

fn main() {
    let sdl = ::rsdl2::Library::new().unwrap();
    let video = sdl.video().unwrap();
    let mut window_size = [640; 2];
    let mut window = video.new_window(str0!("demo - paused"), [::rsdl2::video::WindowPos::Undefined; 2], window_size, ::rsdl2::video::WindowFlags::empty()).unwrap();
    let render = unsafe { use ::core::ops::DerefMut; &mut *(window.deref_mut() as *mut ::rsdl2::video::Window) }.new_renderer(None, ::rsdl2::video::RendererFlags::empty()).unwrap();
    let mut grid_surfaces = [video.new_rgb_surface([grid_size[0] as _, grid_size[1] as _], 1, [1, 1, 1, 0]).unwrap(),
                             video.new_rgb_surface([grid_size[0] as _, grid_size[1] as _], 1, [1, 1, 1, 0]).unwrap()];
    let mut paused = true;
    loop {
        let mut step = false;
        for (ev, _) in video.events(false) { use rsdl2::{event::{Button, Event::*}, key::Sym, window::Event::*}; match ev {
            Quit => return,
            Window(_, Resized(size)) => window_size = size,
            Keyboard { state: true, sym: Sym { sym: 0x0D, .. }, .. } => {
                paused ^= true;
                window.set_title(if paused { str0!("demo - paused") } else { str0!("demo") });
            },
            Keyboard { state: true, sym: Sym { sym: 0x20, .. }, .. } => {
                paused = true;
                step = true;
                window.set_title(str0!("demo - paused"));
            },
            Keyboard { state: true, sym: Sym { sym: 0x65, .. }, .. } => for p in grid_surfaces[0].pixels_mut().raw_bytes() { *p = 0 },
            Keyboard { state: true, sym: Sym { sym: 0x72, .. }, .. } => ::rand::thread_rng().fill_bytes(grid_surfaces[0].pixels_mut().raw_bytes()),
            Pointer { state: true, button, pos, .. } => { let pos = [pos[0] as usize * grid_size[0] / window_size[0] as usize,
                                                                     pos[1] as usize * grid_size[1] / window_size[1] as usize]; match button {
                Button::Left | Button::Right => surface_grid_mut(&mut grid_surfaces[0]).set(grid_size, pos, Button::Left == button),
                Button::Middle => eprintln!("{:02X}@{:?}", surface_grid(&grid_surfaces[0]).get_nbhd(grid_size, pos).0, pos),
                _ => (),
            } },
            _ => (),
        } }
        {
            let t = render.new_texture_from_surface(&grid_surfaces[0]).unwrap();
            render.copy_from(&t, None, None).unwrap();
            render.present();
        }
        if !paused || step {
            grid_surfaces.swap(0, 1);
            let (xs, ys) = grid_surfaces.split_at_mut(1);
            evolve(surface_grid(&ys[0]), surface_grid_mut(&mut xs[0]), grid_size, my_rule);
        }
    }
}

fn my_rule(Neighborhood(xs, _): Neighborhood) -> bool {
    let xs = hexagonalize(xs);
    let c = xs.count_ones();
    3 == xs >> xs.trailing_zeros() || 0b100001 == xs || 4 == c || 5 == c
}

fn surface_grid(s: &::rsdl2::video::Surface) -> &Grid {
    unsafe { ::core::mem::transmute((&mut *(s as *const _ as *mut ::rsdl2::video::Surface)).pixels_mut().raw_bytes()) }
}

fn surface_grid_mut(s: &mut ::rsdl2::video::Surface) -> &mut Grid {
    unsafe { ::core::mem::transmute(s.pixels_mut().raw_bytes()) }
}
