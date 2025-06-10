pub struct LookAndSay;

impl LookAndSay {
    pub fn run(input: &str, iterations: usize) -> String {
        let mut text = input.to_string();

        for _ in 0..iterations {
            let mut buffer = Buffer::default();

            let mut prev = None;
            let mut count = 0;
            for c in text.chars() {
                if let Some(prev) = prev {
                    if c != prev {
                        buffer.append(count, prev);
                        count = 0;
                    }
                }
                prev = Some(c);
                count += 1;
            }
            if let Some(c) = prev {
                buffer.append(count, c);
            }

            text = buffer.text;
        }

        text
    }
}

#[derive(Default)]
struct Buffer {
    text: String,
}

impl Buffer {
    fn append(&mut self, count: u32, c: char) {
        self.text.push(char::from_digit(count, 10).unwrap());
        self.text.push(c);
    }
}
