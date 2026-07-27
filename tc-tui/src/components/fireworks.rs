use std::time::{Duration, Instant};

use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::Widget,
};

const CHARS: &[u8] =
    b"@#$%&*+=^~!?|\\/<>{}[]()0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

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

    fn tick(&mut self, dt: f32, area: Rect) {
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
    burst_colors: [Color; 4],
}

impl FireworksAnimation {
    pub fn new(burst_colors: [Color; 4]) -> Self {
        let pending_launches = vec![(0.20, 0.0), (0.75, 0.8), (0.45, 1.8), (0.60, 3.0)];

        FireworksAnimation {
            particles: Vec::with_capacity(1024),
            rockets: Vec::with_capacity(pending_launches.len()),
            pending_launches,
            elapsed: 0.0,
            started_at: Instant::now(),
            finished: false,
            burst_colors,
        }
    }

    pub fn tick(&mut self, area: Rect) {
        let now = self.started_at.elapsed().as_secs_f32();
        let dt = (now - self.elapsed).min(0.1);
        self.elapsed = now;

        let w = area.width as f32;
        let h = area.height as f32;

        // Launch pending rockets
        let mut launched = Vec::with_capacity(self.pending_launches.len());
        for (i, &(x_frac, launch_t)) in self.pending_launches.iter().enumerate() {
            if self.elapsed >= launch_t {
                let seed = (x_frac * 1000.0) as u32 + (launch_t * 100.0) as u32;
                let color_idx = (x_frac * 100.0) as usize % self.burst_colors.len();
                let target_y = h * (0.10 + lcg_rand(seed) * 0.35);
                self.rockets.push(Rocket {
                    x: x_frac * w,
                    y: h - 1.0,
                    vy: -(2.5 + lcg_rand(seed + 7) * 1.5),
                    target_y,
                    color: self.burst_colors[color_idx],
                    color2: self.burst_colors[(color_idx + 2) % self.burst_colors.len()],
                    trail_accum: 0.0,
                });
                launched.push(i);
            }
        }
        for i in launched.into_iter().rev() {
            self.pending_launches.remove(i);
        }

        // Advance rockets; collect burst positions
        let mut to_burst: Vec<(f32, f32, Color, Color, u32)> =
            Vec::with_capacity(self.rockets.len());
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
                x: rx + (lcg_rand(seed) - 0.5),
                y: ry,
                vx: (lcg_rand(seed + 1) - 0.5) * 0.3,
                vy: 0.15 + lcg_rand(seed + 2) * 0.25,
                color: Color::Yellow,
                ttl: 0.18,
                max_ttl: 0.18,
                glyph_seed: seed,
                trail: Vec::with_capacity(64),
                trail_accum: 0.0,
            });
        }

        for (cx, cy, color, color2, seed) in to_burst {
            self.burst(cx, cy, color, color2, seed);
        }

        for p in &mut self.particles {
            p.tick(dt, area);
        }
        self.particles.retain(Particle::is_alive);

        if self.elapsed >= SHOW_DURATION && self.rockets.is_empty() && self.particles.is_empty() {
            self.finished = true;
        }
    }

    fn burst(&mut self, cx: f32, cy: f32, color: Color, color2: Color, base_seed: u32) {
        let count = 120usize;
        // Scramble base_seed so different burst positions produce genuinely different patterns
        let s0 = avalanche_hash(base_seed);
        for i in 0..count {
            // Chain avalanche_hash through i so each particle is independent
            let s = avalanche_hash(s0.wrapping_add(i as u32));
            let angle = rand_f32(s) * std::f32::consts::TAU;
            // Bimodal speed: dense inner cloud + scattered outer sparks
            let speed = if rand_f32(avalanche_hash(s)) < 0.55 {
                0.15 + rand_f32(avalanche_hash(s.wrapping_add(1))) * 0.9
            } else {
                0.9 + rand_f32(avalanche_hash(s.wrapping_add(2))) * 1.6
            };
            // Compensate for terminal cell aspect ratio (~2:1 height:width)
            let vx = angle.cos() * speed * 2.0;
            let vy = angle.sin() * speed;

            let c = match avalanche_hash(s.wrapping_add(3)) % 4 {
                0 => self.burst_colors[1],
                1 => color,
                2 => color2,
                _ => color,
            };
            let ttl = 0.5 + rand_f32(avalanche_hash(s.wrapping_add(4))) * 1.3;

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
        let buf_area = *buf.area();

        for r in &self.rockets {
            if r.x >= 0.0 && r.y >= 0.0 {
                draw(buf, buf_area, area, r.x as u16, r.y as u16, "|", r.color);
            }
        }

        for p in &self.particles {
            for tp in &p.trail {
                draw(
                    buf,
                    buf_area,
                    area,
                    tp.x,
                    tp.y,
                    rand_char(tp.glyph_seed),
                    trail_color(tp.color, tp.age_frac),
                );
            }
        }

        for p in &self.particles {
            if p.ttl <= 0.0 {
                continue;
            }
            if p.x < 0.0 || p.y < 0.0 {
                continue;
            }
            let life_frac = (p.ttl / p.max_ttl).clamp(0.0, 1.0);
            let color = if life_frac > 0.5 {
                p.color
            } else if life_frac > 0.25 {
                dim_color(p.color)
            } else {
                Color::DarkGray
            };
            draw(
                buf,
                buf_area,
                area,
                p.x as u16,
                p.y as u16,
                rand_char(p.glyph_seed),
                color,
            );
        }
    }
}

fn draw(
    buf: &mut Buffer,
    buf_area: Rect,
    area: Rect,
    x: u16,
    y: u16,
    symbol: &'static str,
    color: Color,
) {
    if x >= area.width || y >= area.height {
        return;
    }
    let ax = area.x.saturating_add(x);
    let ay = area.y.saturating_add(y);
    if ax < buf_area.right() && ay < buf_area.bottom() {
        buf[(ax, ay)]
            .set_symbol(symbol)
            .set_style(Style::default().fg(color));
    }
}

fn rand_char(seed: u32) -> &'static str {
    let idx = (rand_f32(seed) * CHARS.len() as f32) as usize % CHARS.len();
    // SAFETY: CHARS is pure ASCII, so every byte is valid UTF-8
    unsafe { std::str::from_utf8_unchecked(&CHARS[idx..idx + 1]) }
}

fn trail_color(base: Color, age_frac: f32) -> Color {
    if age_frac < 0.3 {
        dim_color(base)
    } else {
        Color::DarkGray
    }
}

fn dim_color(c: Color) -> Color {
    match c {
        Color::Rgb(r, g, b) => Color::Rgb(r / 2, g / 2, b / 2),
        _ => Color::DarkGray,
    }
}

// Avalanche hash - each bit of seed affects all output bits, no stride patterns
fn avalanche_hash(mut x: u32) -> u32 {
    x ^= x >> 17;
    x = x.wrapping_mul(0xbf324c81);
    x ^= x >> 11;
    x = x.wrapping_mul(0x9c7493ad);
    x ^= x >> 15;
    x
}

fn rand_f32(seed: u32) -> f32 {
    avalanche_hash(seed) as f32 / u32::MAX as f32
}

fn lcg_rand(seed: u32) -> f32 {
    let x = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    (x >> 8) as f32 / (1u32 << 24) as f32
}
