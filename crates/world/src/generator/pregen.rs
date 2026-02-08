pub struct Pregenerator {
    pub center_x: i32,
    pub center_z: i32,
    pub radius: i32,

    pub current_r: i32,
    pub current_step: i32,

    pub is_finished: bool,
}

impl Pregenerator {
    pub fn new(radius: i32) -> Self {
        Self {
            center_x: 0,
            center_z: 0,
            radius,
            current_r: 0,
            current_step: 0,
            is_finished: false,
        }
    }

    pub fn next_chunk(&mut self) -> Option<(i32, i32)> {
        if self.current_r > self.radius {
            self.is_finished = true;
            return None;
        }

        if self.current_r == 0 {
            self.current_r += 1;
            return Some((self.center_x, self.center_z));
        }

        let perimeter = 8 * self.current_r;

        if self.current_step >= perimeter {
            self.current_step = 0;
            self.current_r += 1;
            if self.current_r > self.radius {
                self.is_finished = true;
                return None;
            }
        }

        let width = self.radius * 2 + 1;
        let area = width * width;

        if self.current_step >= area {
            self.is_finished = true;
            return None;
        }

        let x = (self.current_step % width) - self.radius;
        let z = (self.current_step / width) - self.radius;

        self.current_step += 1;

        Some((self.center_x + x, self.center_z + z))
    }
}
