use x11::xlib::{
  XOpenDisplay,
  XDefaultScreen,
  XDefaultRootWindow,
  XCreateSimpleWindow,
  XCreateGC,
//  XSelectInput,
//  XMapWindow,
  XMapRaised,
  XClearWindow,
  XGCValues,
//  XSync,
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
  unsafe fn new(path: Option<&str>) -> Result<Display, &'static str> {
    let mut path_ptr = 0 as *mut i8;
    if path.is_some() {
      path_ptr = path.unwrap().as_ptr() as *mut i8;
    }

    let display = XOpenDisplay(path_ptr);

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
  unsafe fn new(display: &'a Display, gcv: GCV) -> Result<Window<'a>, &'static str> {
    let root = XDefaultRootWindow(display.display);
    let window = XCreateSimpleWindow(
      display.display,
      root,
      0, 0,
      200, 100,
      0,
      0,
      0
    );
    let gc = XCreateGC(display.display, window, gcv.flags, gcv.as_ptr());
    if gc.is_null() {
      XDestroyWindow(display.display, window);
      return Err("Failed to create GC");
    }

    XSetWindowBorder(display.display, window, 0x00ff0000);
    XSetWindowBackground(display.display, window, 0x0000ff00);
    XClearWindow(display.display, window);

    Ok(Window {
      window: window,
      display: display,
      gc: gc
    })
  }

  unsafe fn map(&self) {
    XMapRaised(self.display.display, self.window);
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

struct Event<'a> {
  event: XEvent,
  display: &'a Display,
  window: &'a Window<'a>
}

impl<'a> Event<'a> {
  fn new(display: &'a Display, window: &'a Window) -> Event<'a> {
    Event {
      event: XEvent {
	type_: 0
      },
      display: display,
      window: window
    }
  }

  unsafe fn next<F>(&mut self, display: &Display, mut f: F) where
      F: FnMut(&XEvent)
  {
    XNextEvent(display.display, &mut self.event);
    (f)(&self.event);
  }

  unsafe fn select_input(&mut self) {
    XSelectInput(self.display.display, self.window.window,
      ExposureMask     |
      KeyPressMask     |
      KeyReleaseMask   |
      ButtonPressMask
    );
  }
}

fn main() {
  let mut a = 1;

  a = a + 1;

  println!("Hello, world! {}", a);

  unsafe {
    let display = Display::new(None).unwrap_or_else(|e| {
      eprintln!("{}", e);
      std::process::exit(1);
    });

    let _screen = XDefaultScreen(display.display);
    let window = Window::new(&display, GCV::new()).unwrap_or_else(|e| {
      eprintln!("{}", e);
      std::process::exit(1);
    });

    // XSelectInput for later
    // Set border and background of window like this:

    window.map();
//    XSync(display.display, 0);

    let mut ctrl = false;
    let mut running = true;

    let mut event = Event::new(&display, &window);

    event.select_input();

    // Start event loop here
    while running {
      event.next(&display, |e| {
        println!("Event type {}", e.type_);
        // Catch Q key
        if e.type_ == KeyPress {
          println!("Key {} pressed", e.key.keycode);
          match e.key.keycode {
            24 => running = false,             // Q
	    54 => if ctrl { running = false }, // C
	    37 => ctrl = true,                 // Ctrl
             _ => {}
          }
        }
        else if e.type_ == KeyRelease {
	  println!("Key {} released", e.key.keycode);
	  match e.key.keycode {
	    37 => ctrl = false,                // Ctrl
	    _ => {}
	  }
	}
      });
    }
  }
}
