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
  XSync,
  XUnmapWindow,
  XFreeGC,
  XCloseDisplay
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

struct GC {
  gc: x11::xlib::GC
}

struct Display {
  display: *mut x11::xlib::Display,
  gcs: Vec<GC>,
  gcvs: Vec<GCV>
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
        display: display,
	gcs: Vec::new(),
	gcvs: Vec::new()
      })
    }
  }

  unsafe fn gc(&mut self, window: u64, gcv: GCV) -> GC {
    let gc = XCreateGC(self.display, window, gcv.flags, gcv.as_ptr());

    self.gcs.push(GC { gc });
    self.gcvs.push(gcv);

    GC {
      gc: gc
    }
  }
}

impl Drop for Display {
  fn drop(&mut self) {
    unsafe {
      if !self.display.is_null() {
	for gc in self.gcs.iter() {
	  XFreeGC(self.display, gc.gc);
	}
        XCloseDisplay(self.display);
      }
    }
  }
}

fn main() {
  let mut a = 1;

  a = a + 1;

  println!("Hello, world! {}", a);

  unsafe {
    let mut display = Display::new(None).unwrap_or_else(|e| {
      eprintln!("{}", e);
      std::process::exit(1);
    });

    let screen = XDefaultScreen(display.display);
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

    // XSelectInput for later

    let gc = display.gc(window, GCV::new());

    XClearWindow(display.display, window);
    XMapRaised(display.display, window);
    XSync(display.display, 0);
    std::thread::sleep(std::time::Duration::from_secs(5));
    XUnmapWindow(display.display, window);
  }
}
