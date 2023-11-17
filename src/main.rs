use rustx11::librx::{
  Display,
  Window,
  GCV,
  EventHandler,
  Line,
  Shape
};

fn main() {
  let display = Display::new(None).unwrap_or_else(|e| {
    eprintln!("{}", e);
    std::process::exit(1);
  });

  // let _screen = XDefaultScreen(display.display);
  let window = Window::new(&display).unwrap_or_else(|e| {
    eprintln!("{}", e);
    std::process::exit(1);
  });

  window.map();

  let gcv = GCV::new();
  let line = Line::new(&display, &window, 5, 5, 100, 100, &gcv).unwrap();

  line.draw();

  display.flush();
  display.sync();
 
  let mut ctrl = false;
  let mut running = true;
  let mut event_handler = EventHandler::new(&display, &window);

  // TODO : Select what to listen to
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

      line.draw();
    });
  }
}
