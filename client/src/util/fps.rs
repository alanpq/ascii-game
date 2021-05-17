use std::time::Instant;

const FPS_SAMPLES: usize = 5;

pub struct FpsCounter {
    samples: [f32; FPS_SAMPLES],
    sum: f32,
    idx: usize,
    prev: Instant
}

impl FpsCounter {
    pub fn new() -> FpsCounter {
        FpsCounter{
            samples: [0.0; FPS_SAMPLES],
            sum: 0.0,
            idx: 0,
            prev: Instant::now(),
        }
    }

    pub fn tick(&mut self) -> f32 {
        let now = Instant::now();
        self.sum -= self.samples[self.idx];
        let d = now.duration_since(self.prev).as_secs_f32();

        self.sum += d;
        self.samples[self.idx] = d;

        self.idx += 1;
        if self.idx >= FPS_SAMPLES {
            self.idx = 0;
        }
        self.prev = now;
        self.sum / FPS_SAMPLES as f32
    }
}