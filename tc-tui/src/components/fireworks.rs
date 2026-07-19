use std::time::{Duration, Instant};

use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::Widget,
};

// Wide pool of printable ASCII — the source animation uses essentially all of these
const CHARS: &[u8] =
    b"@#$%&*+=^~!?|\\/<>{}[]()0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

const BURST_COLORS: [Color; 8] = [
    Color::Red,
    Color::Yellow,
    Color::Green,
    Color::Cyan,
    Color::Magenta,
    Color::White,
    Color::LightRed,
    Color::LightYellow,
];

const SHOW_DURATION: f32 = 5.0;

// A trail segment: one past position of a particle
struct TrailPoint {
    x: u16,
    y: u16,
    glyph_seed: u32,
    // 0.0 = brand new, 1.0 = fully faded
    age_frac: f32,
    color: Color,
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
    ttl: f32,
    max_ttl: f32,
    glyph_seed: u32,
    // trail of past cell positions
    trail: Vec<TrailPoint>,
    trail_accum: f32,
}

impl Particle {
    fn is_alive(&self) -> bool {
        self.ttl > 0.0 || !self.trail.is_empty()
    }

    fn tick(&mut self, dt: f32) {
        if self.ttl > 0.0 {
            // Emit a trail point ~every cell moved
            self.trail_accum += (self.vx.abs() + self.vy.abs()) * dt * 20.0;
            if self.trail_accum >= 0.8 {
                self.trail_accum = 0.0;
                self.trail.push(TrailPoint {
                    x: self.x as u16,
                    y: self.y as u16,
                    glyph_seed: self.glyph_seed.wrapping_add(self.trail.len() as u32 * 37),
                    age_frac: 0.0,
                    color: self.color,
                });
                // cap trail length so memory stays bounded
                if self.trail.len() > 12 {
                    self.trail.remove(0);
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

struct Rocket {
    x: f32,
    y: f32,
    vy: f32,
    target_y: f32,
    color: Color,
    color2: Color,
    trail_accum: f32,
}

pub struct FireworksAnimation {
    particles: Vec<Particle>,
    rockets: Vec<Rocket>,
    pending_launches: Vec<(f32, f32)>, // (x_frac, launch_at_seconds)
    elapsed: f32,
    started_at: Instant,
    pub finished: bool,
}

impl FireworksAnimation {
    pub fn new() -> Self {
        let pending_launches = vec![(0.20, 0.0), (0.75, 0.8), (0.45, 1.8), (0.60, 3.0)];

        FireworksAnimation {
            particles: Vec::with_capacity(1024),
            rockets: Vec::new(),
            pending_launches,
            elapsed: 0.0,
            started_at: Instant::now(),
            finished: false,
        }
    }

    pub fn tick(&mut self, area: Rect) {
        let now = self.started_at.elapsed().as_secs_f32();
        let dt = (now - self.elapsed).min(0.1);
        self.elapsed = now;

        let w = area.width as f32;
        let h = area.height as f32;

        // Launch pending rockets
        let mut launched = Vec::new();
        for (i, &(x_frac, launch_t)) in self.pending_launches.iter().enumerate() {
            if self.elapsed >= launch_t {
                let seed = (x_frac * 1000.0) as u32 + (launch_t * 100.0) as u32;
                let color_idx = (x_frac * 100.0) as usize % BURST_COLORS.len();
                let target_y = h * (0.10 + pseudo_rand(seed) * 0.35);
                self.rockets.push(Rocket {
                    x: x_frac * w,
                    y: h - 1.0,
                    vy: -(2.5 + pseudo_rand(seed + 7) * 1.5),
                    target_y,
                    color: BURST_COLORS[color_idx],
                    color2: BURST_COLORS[(color_idx + 3) % BURST_COLORS.len()],
                    trail_accum: 0.0,
                });
                launched.push(i);
            }
        }
        for i in launched.into_iter().rev() {
            self.pending_launches.remove(i);
        }

        // Advance rockets; collect burst positions
        let mut to_burst: Vec<(f32, f32, Color, Color, u32)> = Vec::new();
        self.rockets.retain_mut(|r| {
            r.y += r.vy * dt * 20.0;
            r.vy *= 1.0 - (0.03 * dt * 20.0).min(0.99);

            r.trail_accum += dt;
            if r.y <= r.target_y || r.vy.abs() < 0.1 {
                let seed = r.x as u32 * 17 + r.y as u32 * 31;
                to_burst.push((r.x, r.y, r.color, r.color2, seed));
                false
            } else {
                true
            }
        });

        // Rocket trail sparks
        let trail_data: Vec<(f32, f32, u32)> = self
            .rockets
            .iter()
            .map(|r| (r.x, r.y, (self.elapsed * 1000.0) as u32 + r.x as u32))
            .collect();
        for (rx, ry, seed) in trail_data {
            self.particles.push(Particle {
                x: rx + (pseudo_rand(seed) - 0.5),
                y: ry,
                vx: (pseudo_rand(seed + 1) - 0.5) * 0.3,
                vy: 0.15 + pseudo_rand(seed + 2) * 0.25,
                color: Color::Yellow,
                ttl: 0.18,
                max_ttl: 0.18,
                glyph_seed: seed,
                trail: Vec::new(),
                trail_accum: 0.0,
            });
        }

        for (cx, cy, color, color2, seed) in to_burst {
            self.burst(cx, cy, color, color2, seed);
        }

        for p in &mut self.particles {
            p.tick(dt);
        }
        self.particles.retain(Particle::is_alive);

        if self.elapsed >= SHOW_DURATION && self.rockets.is_empty() && self.particles.is_empty() {
            self.finished = true;
        }
    }

    fn burst(&mut self, cx: f32, cy: f32, color: Color, color2: Color, base_seed: u32) {
        let count = 120usize;
        // Scramble base_seed so different burst positions produce genuinely different patterns
        let s0 = hash(base_seed);
        for i in 0..count {
            // Chain hash through i so each particle is independent
            let s = hash(s0.wrapping_add(i as u32));
            let angle = hf(s) * std::f32::consts::TAU;
            // Bimodal speed: dense inner cloud + scattered outer sparks
            let speed = if hf(hash(s)) < 0.55 {
                0.15 + hf(hash(s.wrapping_add(1))) * 0.9
            } else {
                0.9 + hf(hash(s.wrapping_add(2))) * 1.6
            };
            // Compensate for terminal cell aspect ratio (~2:1 height:width)
            let vx = angle.cos() * speed * 2.0;
            let vy = angle.sin() * speed;

            let c = match hash(s.wrapping_add(3)) % 4 {
                0 => Color::White,
                1 => color,
                2 => color2,
                _ => color,
            };
            let ttl = 0.5 + hf(hash(s.wrapping_add(4))) * 1.3;

            self.particles.push(Particle {
                x: cx,
                y: cy,
                vx,
                vy,
                color: c,
                ttl,
                max_ttl: ttl,
                glyph_seed: s,
                trail: Vec::with_capacity(12),
                trail_accum: 0.0,
            });
        }
    }

    pub fn duration() -> Duration {
        Duration::from_secs_f32(SHOW_DURATION)
    }
}

impl Widget for &mut FireworksAnimation {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Draw rockets
        for r in &self.rockets {
            let x = r.x as u16;
            let y = r.y as u16;
            if in_bounds(x, y, area) {
                buf[(x, y)]
                    .set_symbol("|")
                    .set_style(Style::default().fg(Color::White));
            }
        }

        // Draw particle trails first (underneath the particle head)
        for p in &self.particles {
            for tp in &p.trail {
                if in_bounds(tp.x, tp.y, area) {
                    let color = trail_color(tp.color, tp.age_frac);
                    buf[(tp.x, tp.y)]
                        .set_symbol(rand_char(tp.glyph_seed))
                        .set_style(Style::default().fg(color));
                }
            }
        }

        // Draw particle heads on top
        for p in &self.particles {
            if p.ttl <= 0.0 {
                continue;
            }
            let x = p.x as u16;
            let y = p.y as u16;
            if in_bounds(x, y, area) {
                let life_frac = (p.ttl / p.max_ttl).clamp(0.0, 1.0);
                let color = if life_frac > 0.5 {
                    p.color
                } else if life_frac > 0.25 {
                    dim_color(p.color)
                } else {
                    Color::DarkGray
                };
                buf[(x, y)]
                    .set_symbol(rand_char(p.glyph_seed))
                    .set_style(Style::default().fg(color));
            }
        }
    }
}

fn in_bounds(x: u16, y: u16, area: Rect) -> bool {
    x >= area.left() && x < area.right() && y >= area.top() && y < area.bottom()
}

fn rand_char(seed: u32) -> &'static str {
    let idx = (hf(seed) * CHARS.len() as f32) as usize % CHARS.len();
    // SAFETY: CHARS is pure ASCII, so every byte is valid UTF-8
    unsafe { std::str::from_utf8_unchecked(&CHARS[idx..idx + 1]) }
}

fn trail_color(base: Color, age_frac: f32) -> Color {
    if age_frac < 0.3 {
        dim_color(base)
    } else if age_frac < 0.6 {
        Color::DarkGray
    } else {
        Color::DarkGray
    }
}

fn dim_color(c: Color) -> Color {
    match c {
        Color::Red | Color::LightRed => Color::Red,
        Color::Yellow | Color::LightYellow => Color::Yellow,
        Color::Green => Color::Green,
        Color::Cyan => Color::Cyan,
        Color::Magenta => Color::Magenta,
        Color::White => Color::Gray,
        _ => Color::DarkGray,
    }
}

// Avalanche hash — each bit of seed affects all output bits, no stride patterns
fn hash(mut x: u32) -> u32 {
    x ^= x >> 17;
    x = x.wrapping_mul(0xbf324c81);
    x ^= x >> 11;
    x = x.wrapping_mul(0x9c7493ad);
    x ^= x >> 15;
    x
}

fn hf(seed: u32) -> f32 {
    hash(seed) as f32 / u32::MAX as f32
}

// Keep pseudo_rand for seeding rockets/launches where uniformity is fine
fn pseudo_rand(seed: u32) -> f32 {
    let x = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    (x >> 8) as f32 / (1u32 << 24) as f32
}
