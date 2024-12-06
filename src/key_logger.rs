use rdev::{EventType, Key};

pub struct KeyLogger {
    tx_key: std::sync::mpsc::Sender<(Key, EventType)>,
}

impl KeyLogger {
    pub fn new(tx_key: std::sync::mpsc::Sender<(Key, EventType)>) -> Self {
        Self { tx_key }
    }

    pub fn listen(self) {
        // TODO: Implement better differentiation between key press and key release for different
        // keys. For example, we dont need to keep sending KeyPress for Shift while it is held
        // down, but only once when it is pressed and once when it is released.
        //
        // Any further tools like text extraction, etc. can be implemented separately. This
        // function should only be responsible for sending the key events.

        rdev::listen(move |event| match event.event_type {
            EventType::KeyPress(key) | EventType::KeyRelease(key) => {
                self.tx_key.send((key, event.event_type)).unwrap();
                return;
            }
            _ => (),
        })
        .unwrap();
    }
}
