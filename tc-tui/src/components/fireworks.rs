use std::time::{Duration, Instant};

use chrono::{DateTime, Datelike, Local, Timelike};
use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::Widget,
};
use tc_models::color_theme::ThemeColor;

use crate::{
    assets,
    tui_models::{
        firework::{Particle, Rocket},
        tui_constants::{FIREWORK_ART_CHARS, FIREWORK_PROJECTILE_TTL_S},
    },
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FireworksPhase {
    Idle,
    Playing,
    Finished,
}

pub struct FireworksAnimation {
    particles: Vec<Particle>,
    rockets: Vec<Rocket>,
    pending_launches: Vec<(f32, f32)>,
    elapsed: f32,
    last_tick: Instant,
    phase: FireworksPhase,
    /// Which year's celebration was already shown, so the 10s window can't retrigger it
    shown_for_year: Option<i32>,
}

impl FireworksAnimation {
    pub fn new() -> Self {
        Self {
            particles: Vec::with_capacity(1024),
            rockets: Vec::new(),
            pending_launches: Vec::new(),
            elapsed: 0.0,
            last_tick: Instant::now(),
            phase: FireworksPhase::Idle,
            shown_for_year: None,
        }
    }

    pub fn start(&mut self, year: i32) {
        self.pending_launches = vec![(0.20, 0.0), (0.75, 0.8), (0.45, 1.8), (0.60, 3.0)];
        self.particles.clear();
        self.rockets.clear();
        self.elapsed = 0.0;
        self.last_tick = Instant::now();
        self.shown_for_year = Some(year);
        self.phase = FireworksPhase::Playing;
    }

    /// Lets any keypress skip the animation
    pub fn skip(&mut self) {
        self.phase = FireworksPhase::Finished;
    }

    pub fn is_playing(&self) -> bool {
        self.phase == FireworksPhase::Playing
    }

    pub fn is_finished(&self) -> bool {
        self.phase == FireworksPhase::Finished
    }

    // Trigger window: Dec 31 23:59:55 -> Jan 1 00:00:05
    pub fn pending_trigger(&self, now: DateTime<Local>) -> Option<i32> {
        // NOTE: this is a setup for testing the fireworks
        let force = std::env::var_os("TC_FORCE_FIREWORKS").is_some();
        let year = if force {
            now.year()
        } else if now.month() == 12
            && now.day() == 31
            && now.hour() == 23
            && now.minute() == 59
            && now.second() >= 55
        {
            now.year() + 1
        } else if now.month() == 1
            && now.day() == 1
            && now.hour() == 0
            && now.minute() == 0
            && now.second() <= 5
        {
            now.year()
        } else {
            return None;
        };
        (self.shown_for_year != Some(year) && !self.is_playing()).then_some(year)
    }

    pub fn get_burst_colors(color_theme_idx: u16) -> [Color; 4] {
        let theme = assets().get_color_theme(color_theme_idx);

        [
            *theme.get(&ThemeColor::Accent),
            *theme.get(&ThemeColor::Foreground),
            *theme.get(&ThemeColor::Selection),
            *theme.get(&ThemeColor::Borders),
        ]
    }

    pub fn tick(&mut self, area: Rect, burst_colors: [Color; 4]) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_tick).as_secs_f32().min(0.1);
        self.last_tick = now;
        self.elapsed += dt;

        let w = area.width as f32;
        let h = area.height as f32;

        // Launch pending rockets
        let mut launched = Vec::with_capacity(self.pending_launches.len());
        for (i, &(x_frac, launch_t)) in self.pending_launches.iter().enumerate() {
            if self.elapsed >= launch_t {
                let seed = (x_frac * 1000.0) as u32 + (launch_t * 100.0) as u32;
                let color_idx = (x_frac * 100.0) as usize % burst_colors.len();
                let target_y = h * (0.10 + Self::lcg_rand(seed) * 0.35);
                self.rockets.push(Rocket {
                    x: x_frac * w,
                    y: h - 1.0,
                    vy: -(2.5 + Self::lcg_rand(seed + 7) * 1.5),
                    target_y,
                    color: burst_colors[color_idx],
                    color2: burst_colors[(color_idx + 2) % burst_colors.len()],
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
                x: rx + (Self::lcg_rand(seed) - 0.5),
                y: ry,
                vx: (Self::lcg_rand(seed + 1) - 0.5) * 0.3,
                vy: 0.15 + Self::lcg_rand(seed + 2) * 0.25,
                color: Color::Yellow,
                ttl: 0.18,
                max_ttl: 0.18,
                glyph_seed: seed,
                trail: Vec::with_capacity(64),
                trail_accum: 0.0,
            });
        }

        for (cx, cy, color, color2, seed) in to_burst {
            self.burst(cx, cy, color, color2, seed, burst_colors);
        }

        for p in &mut self.particles {
            p.tick(dt, area);
        }
        self.particles.retain(Particle::is_alive);

        if Duration::from_secs_f32(self.elapsed) >= FIREWORK_PROJECTILE_TTL_S
            && self.rockets.is_empty()
            && self.particles.is_empty()
        {
            self.phase = FireworksPhase::Finished;
        }
    }

    fn burst(
        &mut self,
        cx: f32,
        cy: f32,
        color: Color,
        color2: Color,
        base_seed: u32,
        burst_colors: [Color; 4],
    ) {
        let count = 120usize;
        // Scramble base_seed so different burst positions produce genuinely different patterns
        let s0 = Self::avalanche_hash(base_seed);
        for i in 0..count {
            // Chain avalanche_hash through i so each particle is independent
            let s = Self::avalanche_hash(s0.wrapping_add(i as u32));
            let angle = Self::rand_f32(s) * std::f32::consts::TAU;
            // Bimodal speed: dense inner cloud + scattered outer sparks
            let speed = if Self::rand_f32(Self::avalanche_hash(s)) < 0.55 {
                0.15 + Self::rand_f32(Self::avalanche_hash(s.wrapping_add(1))) * 0.9
            } else {
                0.9 + Self::rand_f32(Self::avalanche_hash(s.wrapping_add(2))) * 1.6
            };
            // Compensate for terminal cell aspect ratio (~2:1 height:width)
            let vx = angle.cos() * speed * 2.0;
            let vy = angle.sin() * speed;

            let c = match Self::avalanche_hash(s.wrapping_add(3)) % 4 {
                0 => burst_colors[1],
                1 => color,
                2 => color2,
                _ => color,
            };
            let ttl = 0.5 + Self::rand_f32(Self::avalanche_hash(s.wrapping_add(4))) * 1.3;

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
        FIREWORK_PROJECTILE_TTL_S
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
        let idx = (Self::rand_f32(seed) * FIREWORK_ART_CHARS.len() as f32) as usize
            % FIREWORK_ART_CHARS.len();
        // SAFETY: CHARS is pure ASCII, so every byte is valid UTF-8
        unsafe { std::str::from_utf8_unchecked(&FIREWORK_ART_CHARS[idx..idx + 1]) }
    }

    fn trail_color(base: Color, age_frac: f32) -> Color {
        if age_frac < 0.3 {
            Self::dim_color(base)
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
        Self::avalanche_hash(seed) as f32 / u32::MAX as f32
    }

    fn lcg_rand(seed: u32) -> f32 {
        let x = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        (x >> 8) as f32 / (1u32 << 24) as f32
    }
}

impl Widget for &FireworksAnimation {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let buf_area = *buf.area();

        for r in &self.rockets {
            if r.x >= 0.0 && r.y >= 0.0 {
                FireworksAnimation::draw(buf, buf_area, area, r.x as u16, r.y as u16, "|", r.color);
            }
        }

        for p in &self.particles {
            for tp in &p.trail {
                FireworksAnimation::draw(
                    buf,
                    buf_area,
                    area,
                    tp.x,
                    tp.y,
                    FireworksAnimation::rand_char(tp.glyph_seed),
                    FireworksAnimation::trail_color(tp.color, tp.age_frac),
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
                FireworksAnimation::dim_color(p.color)
            } else {
                Color::DarkGray
            };
            FireworksAnimation::draw(
                buf,
                buf_area,
                area,
                p.x as u16,
                p.y as u16,
                FireworksAnimation::rand_char(p.glyph_seed),
                color,
            );
        }
    }
}
