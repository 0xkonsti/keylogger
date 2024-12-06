use rdev::{EventType, Key};

pub struct KeyLogger {
    tx: std::sync::mpsc::Sender<String>,
}

impl KeyLogger {
    pub fn new(tx: std::sync::mpsc::Sender<String>) -> Self {
        Self { tx }
    }

    pub fn listen(self) {
        let mut cursor = 0;
        let mut word = 0;
        let mut words = vec!["".to_string()];

        rdev::listen(move |event| {
            let key = match event.event_type {
                EventType::KeyPress(key) => key,
                _ => return,
            };

            match key {
                Key::Space => {
                    self.tx.send(words[word].clone()).unwrap();
                    cursor = 0;
                    word += 1;
                    words.push("".to_string());
                }
                Key::LeftArrow => {
                    if cursor > 0 {
                        cursor -= 1;
                    } else if word > 0 {
                        word -= 1;
                        cursor = words[word].len();
                    }
                }
                Key::RightArrow => {
                    if cursor < words[word].len() {
                        cursor += 1;
                    } else if word < words.len() - 1 {
                        word += 1;
                        cursor = 0;
                    }
                }
                Key::Backspace | Key::Delete => {
                    // TODO: Implement backspace and delete
                }
                Key::Return => {
                    // TODO: Implement return
                }
                _ => {
                    if let Some(name) = event.name {
                        if name.bytes().last() < Some(127_u8) && name.bytes().last() > Some(31_u8) {
                            for c in name.chars() {
                                words[word].insert(cursor, c);
                                cursor += 1;
                            }
                        }
                    }
                }
            };
        })
        .unwrap();
    }
}
