pub mod librx {

use x11::xlib::{
  XOpenDisplay,
  //XDefaultScreen,
  XDefaultRootWindow,
  XCreateSimpleWindow,
  XCreateGC,
  XMapRaised,
  XClearWindow,
  XGCValues,
  XUnmapWindow,
  XDestroyWindow,
  XFreeGC,
  XCloseDisplay,
  XSetWindowBorder,
  XSetWindowBackground,
  XNextEvent,
  XEvent,
  XSelectInput,
  ExposureMask,
  KeyPress,
  KeyPressMask,
  KeyRelease,
  KeyReleaseMask,
  ButtonPressMask,
  XDrawLine,
//  GCForeground,
//  GCLineWidth,
  XSync,
  XFlush
};

#[derive(Clone)]
pub struct GCV {
  flags: u64,
  gcv: Box<XGCValues>
}

impl GCV {
  pub fn new() -> GCV {
    GCV {
      //flags: (GCForeground | GCLineWidth) as u64,
      flags: 0,
      gcv: Box::new(XGCValues {
	function: 0,
	plane_mask: 0,
	foreground: 0,
	background: 0,
	line_width: 10,
	line_style: 0,
	cap_style: 0,
	join_style: 0,
	fill_style: 0,
	fill_rule: 0,
	arc_mode: 0,
	tile: 0,
	stipple: 0,
	ts_x_origin: 0,
	ts_y_origin: 0,
	font: 0,
	subwindow_mode: 0,
	graphics_exposures: 0,
	clip_x_origin: 0,
	clip_y_origin: 0,
	clip_mask: 0,
	dash_offset: 0,
	dashes: 0
      })
    }
  }

  fn as_ptr(&self) -> *mut XGCValues {
    &*self.gcv as *const XGCValues as *mut XGCValues
  }
}

struct GC<'a> {
  gc: x11::xlib::GC,
  display: &'a Display
}

impl<'a> GC<'a> {
  fn new(display: &'a Display, window: &'a Window, gcv: &GCV) -> Result<GC<'a>, &'static str> {
    let gc: x11::xlib::GC;
    unsafe {
      gc = XCreateGC(display.display, window.window, gcv.flags, gcv.as_ptr());
    }
    if gc.is_null() {
      Err("Failed to create GC")
    }
    else {
      Ok(GC { gc, display })
    }
  }
}

impl<'a> Drop for GC<'a> {
  fn drop(&mut self) {
    unsafe {
      XFreeGC(self.display.display, self.gc);
    }
  }
}

pub struct Display {
  display: *mut x11::xlib::Display
}

impl Display {
  pub fn new(path: Option<&str>) -> Result<Display, &'static str> {
    let mut path_ptr = 0 as *mut i8;
    if path.is_some() {
      path_ptr = path.unwrap().as_ptr() as *mut i8;
    }

    let display: *mut x11::xlib::Display;

    unsafe {
      display = XOpenDisplay(path_ptr);
    }

    if display.is_null() {
      Err("Failed to open display")
    }
    else {
      Ok(Display {
	display: display
      })
    }
  }

  pub fn sync(&self) {
    unsafe {
      XSync(self.display, 0);
    }
  }

  pub fn flush(&self) {
    unsafe {
      XFlush(self.display);
    }
  }
}

impl Drop for Display {
  fn drop(&mut self) {
    unsafe {
      XCloseDisplay(self.display);
    }
  }
}

pub trait Shape {
  fn draw(&self);
}

pub struct Line<'a> {
  x1: i32,
  y1: i32,
  x2: i32,
  y2: i32,
  gc: GC<'a>,
  display: &'a Display,
  window: &'a Window<'a>
}

impl<'a> Line<'a> {
  pub fn new(display: &'a Display, window: &'a Window, x1: i32, y1: i32, x2: i32, y2: i32, gcv: &GCV)
    -> Result<Line<'a>, &'static str> {
    Ok(Line {
      x1: x1,
      y1: y1,
      x2: x2,
      y2: y2,
      // TODO : Try to handle error
      gc: GC::new(display, window, gcv).unwrap(),
      display: display,
      window: window
    })
  }
}

impl<'a> Shape for Line<'a> {
  fn draw(&self) {
    unsafe {
      XDrawLine(self.display.display, self.window.window, self.gc.gc, self.x1, self.y1, self.x2, self.y2);
    }
  }
}

pub struct Window<'a> {
  window: u64,
  display: &'a Display
}

impl<'a> Window<'a> {
  pub fn new(display: &'a Display) -> Result<Window<'a>, &'static str> {
    let root: u64;
    let window: u64;
    
    unsafe {
      root = XDefaultRootWindow(display.display);
      window = XCreateSimpleWindow(
	display.display,
	root,
	0, 0,
	200, 100,
	0,
	0,
	0
      );

      XSetWindowBorder(display.display, window, 0x00ff0000);
      XSetWindowBackground(display.display, window, 0x0000ff00);
      XClearWindow(display.display, window);
    }

    Ok(Window {
      window: window,
      display: display
    })
  }

  pub fn map(&self) {
    unsafe {
      XMapRaised(self.display.display, self.window);
    }
  }
}

impl<'a> Drop for Window<'a> {
  fn drop(&mut self) {
    unsafe {
      XUnmapWindow(self.display.display, self.window);
      XDestroyWindow(self.display.display, self.window);
    }
  }
}

pub struct Event {
  e: XEvent
}

impl Event {
  fn get_type(&self) -> i32 {
    unsafe {
      return self.e.type_;
    }
  }

  pub fn get_key_press_code(&self) -> Option<u32> {
    if self.get_type() != KeyPress {
      return None;
    }

    unsafe {
      return Some(self.e.key.keycode);
    }
  }

  pub fn get_key_release_code(&self) -> Option<u32> {
    if self.get_type() != KeyRelease {
      return None;
    }

    unsafe {
      return Some(self.e.key.keycode);
    }
  }
}

pub struct EventHandler<'a> {
  display: &'a Display,
  window: &'a Window<'a>
}

impl<'a> EventHandler<'a> {
  pub fn new(display: &'a Display, window: &'a Window) -> EventHandler<'a> {
    EventHandler {
      display: display,
      window: window
    }
  }

  pub fn next<F>(&mut self, display: &Display, mut f: F) where
      F: FnMut(&Event)
  {
    let mut e = XEvent { type_: 0 };
    unsafe {
      XNextEvent(display.display, &mut e);
      println!("Event type {}", e.type_);
    }
    (f)(&Event{e});
  }

  pub fn select_input(&mut self) {
    unsafe {
      XSelectInput(self.display.display, self.window.window,
	ExposureMask     |
	KeyPressMask     |
	KeyReleaseMask   |
	ButtonPressMask
      );
    }
  }
}

} /* mod librx */
