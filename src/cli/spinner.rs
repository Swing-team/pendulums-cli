use spinners::{Spinner, Spinners};

pub struct PendulumsSpinner {
    sp: Spinner,
}

impl PendulumsSpinner {
    pub fn start() -> PendulumsSpinner{
      return PendulumsSpinner {
        sp: Spinner::new(Spinners::Aesthetic, "".into())
      }
    }

    pub fn stop(&mut self) {
        self.sp.stop();
        print!("\x1b[2K\r")
    }
}
