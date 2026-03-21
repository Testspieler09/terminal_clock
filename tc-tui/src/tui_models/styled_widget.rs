use ratatui::{buffer::Buffer, layout::Rect};

pub(crate) trait StyledWidget {
    type Context<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, ctx: Self::Context<'_>);
}
