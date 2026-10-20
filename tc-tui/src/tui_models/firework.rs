use ratatui::{prelude::Rect, style::Color};

// A trail segment: one past position of a particle
pub struct TrailPoint {
    pub x: u16,
    pub y: u16,
    pub glyph_seed: u32,
    // 0.0 = brand new, 1.0 = fully faded
    pub age_frac: f32,
    pub color: Color,
}

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: Color,
    pub ttl: f32,
    pub max_ttl: f32,
    pub glyph_seed: u32,
    // trail of past cell positions
    pub trail: Vec<TrailPoint>,
    pub trail_accum: f32,
}

impl Particle {
    pub fn is_alive(&self) -> bool {
        self.ttl > 0.0 || !self.trail.is_empty()
    }

    pub fn tick(&mut self, dt: f32, area: Rect) {
        let w = area.width as f32;
        let h = area.height as f32;
        if self.ttl > 0.0 {
            // Emit a trail point ~every cell moved, only when inside the area
            self.trail_accum += (self.vx.abs() + self.vy.abs()) * dt * 20.0;
            if self.trail_accum >= 0.8 {
                self.trail_accum = 0.0;
                if self.x >= 0.0 && self.x < w && self.y >= 0.0 && self.y < h {
                    self.trail.push(TrailPoint {
                        x: self.x as u16,
                        y: self.y as u16,
                        glyph_seed: self.glyph_seed.wrapping_add(self.trail.len() as u32 * 37),
                        age_frac: 0.0,
                        color: self.color,
                    });
                }
            }

            self.x += self.vx * dt * 20.0;
            self.y += self.vy * dt * 20.0;
            self.vy += 0.12 * dt * 20.0; // gravity
            self.vx *= 1.0 - (0.04 * dt * 20.0).min(0.95); // drag
            self.ttl -= dt;
        }

        // Age trail points; remove fully faded ones
        let fade_rate = dt / 0.5; // trail fades out over ~0.5s
        for tp in &mut self.trail {
            tp.age_frac = (tp.age_frac + fade_rate).min(1.0);
        }
        self.trail.retain(|tp| tp.age_frac < 1.0);
    }
}

pub struct Rocket {
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub target_y: f32,
    pub color: Color,
    pub color2: Color,
    pub trail_accum: f32,
}
