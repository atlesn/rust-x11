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
  ButtonPressMask
};

struct GCV {
  flags: u64,
  gcv: Box<XGCValues>
}

impl GCV {
  fn new() -> GCV {
    GCV {
      flags: 0,
      gcv: Box::new(XGCValues {
	function: 0,
	plane_mask: 0,
	foreground: 0,
	background: 0,
	line_width: 0,
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

struct Display {
  display: *mut x11::xlib::Display
}

impl Display {
  fn new(path: Option<&str>) -> Result<Display, &'static str> {
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
}

impl Drop for Display {
  fn drop(&mut self) {
    unsafe {
      XCloseDisplay(self.display);
    }
  }
}

struct Window<'a> {
  window: u64,
  display: &'a Display,
  gc: x11::xlib::GC
}

impl<'a> Window<'a> {
  fn new(display: &'a Display, gcv: GCV) -> Result<Window<'a>, &'static str> {
    let root: u64;
    let window: u64;
    let gc: x11::xlib::GC;
    
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

      gc = XCreateGC(display.display, window, gcv.flags, gcv.as_ptr());

      if gc.is_null() {
	XDestroyWindow(display.display, window);
	return Err("Failed to create GC");
      }

      XSetWindowBorder(display.display, window, 0x00ff0000);
      XSetWindowBackground(display.display, window, 0x0000ff00);
      XClearWindow(display.display, window);
    }

    Ok(Window {
      window: window,
      display: display,
      gc: gc
    })
  }

  fn map(&self) {
    unsafe {
      XMapRaised(self.display.display, self.window);
    }
  }
}

impl<'a> Drop for Window<'a> {
  fn drop(&mut self) {
    unsafe {
      XFreeGC(self.display.display, self.gc);
      XUnmapWindow(self.display.display, self.window);
      XDestroyWindow(self.display.display, self.window);
    }
  }
}

struct Event {
  e: XEvent
}

impl Event {
  fn get_type(&self) -> i32 {
    unsafe {
      return self.e.type_;
    }
  }

  fn get_key_press_code(&self) -> Option<u32> {
    if self.get_type() != KeyPress {
      return None;
    }

    unsafe {
      return Some(self.e.key.keycode);
    }
  }

  fn get_key_release_code(&self) -> Option<u32> {
    if self.get_type() != KeyRelease {
      return None;
    }

    unsafe {
      return Some(self.e.key.keycode);
    }
  }
}

struct EventHandler<'a> {
  display: &'a Display,
  window: &'a Window<'a>
}

impl<'a> EventHandler<'a> {
  fn new(display: &'a Display, window: &'a Window) -> EventHandler<'a> {
    EventHandler {
      display: display,
      window: window
    }
  }

  fn next<F>(&mut self, display: &Display, mut f: F) where
      F: FnMut(&Event)
  {
    let mut e = XEvent { type_: 0 };
    unsafe {
      XNextEvent(display.display, &mut e);
      println!("Event type {}", e.type_);
    }
    (f)(&Event{e});
  }

  fn select_input(&mut self) {
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

fn main() {
  let display = Display::new(None).unwrap_or_else(|e| {
    eprintln!("{}", e);
    std::process::exit(1);
  });

  // let _screen = XDefaultScreen(display.display);
  let window = Window::new(&display, GCV::new()).unwrap_or_else(|e| {
    eprintln!("{}", e);
    std::process::exit(1);
  });

  window.map();

  let mut ctrl = false;
  let mut running = true;
  let mut event_handler = EventHandler::new(&display, &window);

  event_handler.select_input();

  while running {
    event_handler.next(&display, |e| {
      e.get_key_press_code().map(|code| {
	println!("Key {} pressed", code);
	match code {
	  24 => running = false,             // Q
	  54 => if ctrl { running = false }, // C
	  37 => ctrl = true,                 // Ctrl
	   _ => {}
	}
      });

      e.get_key_release_code().map(|code| {
	println!("Key {} released", code);
	match code {
	  37 => ctrl = false,                // Ctrl
	  _ => {}
	}
      });
    });
  }
}
