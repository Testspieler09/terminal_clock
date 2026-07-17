use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::Widget,
};

const GLYPHS: [&str; 3] = ["▪", "*", "·"];
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

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
    ttl: u8,
    glyph: &'static str,
}

impl Particle {
    fn is_alive(&self) -> bool {
        self.ttl > 0
    }

    fn tick(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.vy += 0.12; // gravity
        self.vx *= 0.97; // air drag
        self.ttl = self.ttl.saturating_sub(1);
    }
}

pub struct FireworksAnimation {
    particles: Vec<Particle>,
    // rockets waiting to launch: (x frac 0..1, launch_at_frame)
    pending_rockets: Vec<(f32, u32)>,
    frame: u32,
    pub finished: bool,
    total_frames: u32,
}

impl FireworksAnimation {
    pub fn new() -> Self {
        // Stagger 6 rockets across the animation duration
        let pending_rockets = vec![
            (0.15, 0),
            (0.75, 8),
            (0.45, 16),
            (0.25, 24),
            (0.65, 30),
            (0.50, 38),
        ];

        FireworksAnimation {
            particles: Vec::with_capacity(256),
            pending_rockets,
            frame: 0,
            finished: false,
            total_frames: 90,
        }
    }

    pub fn tick(&mut self, area: Rect) {
        self.frame += 1;

        // Launch pending rockets whose frame has arrived
        let w = area.width as f32;
        let h = area.height as f32;

        let mut launched = Vec::new();
        for (i, &(x_frac, launch_frame)) in self.pending_rockets.iter().enumerate() {
            if self.frame >= launch_frame {
                Self::burst(
                    &mut self.particles,
                    x_frac * w,
                    // burst height: upper 20–60% of the terminal
                    h * (0.2 + pseudo_rand(self.frame + i as u32) * 0.4),
                );
                launched.push(i);
            }
        }
        // remove launched rockets (iterate in reverse to preserve indices)
        for i in launched.into_iter().rev() {
            self.pending_rockets.remove(i);
        }

        // Tick all particles
        for p in &mut self.particles {
            p.tick();
        }
        self.particles.retain(Particle::is_alive);

        if self.frame >= self.total_frames && self.particles.is_empty() {
            self.finished = true;
        }
    }

    fn burst(particles: &mut Vec<Particle>, cx: f32, cy: f32) {
        let color = BURST_COLORS[(cx as usize + cy as usize) % BURST_COLORS.len()];
        let color2 = BURST_COLORS[(cx as usize + cy as usize + 3) % BURST_COLORS.len()];
        let count = 24usize;

        for i in 0..count {
            let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
            // terminals are ~2× taller per cell than wide, compensate horizontally
            let speed = 0.6 + pseudo_rand(i as u32 + cx as u32) * 0.8;
            particles.push(Particle {
                x: cx,
                y: cy,
                vx: angle.cos() * speed * 1.8,
                vy: angle.sin() * speed,
                color: if i % 2 == 0 { color } else { color2 },
                ttl: 18 + (pseudo_rand(i as u32 * 7) * 10.0) as u8,
                glyph: GLYPHS[i % GLYPHS.len()],
            });
        }
    }
}

impl Widget for &mut FireworksAnimation {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for p in &self.particles {
            let x = p.x as u16;
            let y = p.y as u16;
            if x >= area.left() && x < area.right() && y >= area.top() && y < area.bottom() {
                // Fade color towards dark as TTL decreases
                let intensity = (p.ttl as f32 / 28.0).clamp(0.0, 1.0);
                let color = if intensity < 0.35 {
                    Color::DarkGray
                } else {
                    p.color
                };
                buf[(x, y)]
                    .set_symbol(p.glyph)
                    .set_style(Style::default().fg(color));
            }
        }
    }
}

// Deterministic cheap pseudo-random in 0..1, no external dep needed
fn pseudo_rand(seed: u32) -> f32 {
    let x = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    (x >> 8) as f32 / (1u32 << 24) as f32
}
