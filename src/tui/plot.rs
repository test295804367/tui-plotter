
use tui::{
    layout::Rect,
    widgets::{Block, Widget},
    style::{Color, Style},
    Frame,
};

pub struct PlotWidget {
    data: Vec<(f64, f64)>,
    block: Option<Block<'static>>,
}

impl PlotWidget {
    pub fn new(data: Vec<(f64, f64)>) -> PlotWidget {
        PlotWidget { data, block: None }
    }

    pub fn block(mut self, block: Block<'static>) -> Self {
        self.block = Some(block);
        self
    }
}

impl Widget for PlotWidget {
    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer) {
        let PlotWidget { data, block } = self;

        // Render block if it exists
        if let Some(block) = block {
            let inner_area = block.inner(area);
            block.render(area, buf);
            self.draw_plot(inner_area, buf);
        } else {
            self.draw_plot(area, buf);
        }
    }
}

impl PlotWidget {
    fn draw_plot(&self, area: Rect, buf: &mut tui::buffer::Buffer) {
        let mut plot = String::new();

        // Plotting the data in ASCII
        for (x, y) in &self.data {
            let x_scaled = ((*x + 5.0) / 10.0 * (area.width as f64)).round() as usize;
            let y_scaled = ((5.0 - y) / 10.0 * (area.height as f64)).round() as usize;
            if x_scaled < area.width as usize && y_scaled < area.height as usize {
                plot.push_str(&format!("({}, {}) ", x_scaled, y_scaled));
            }
        }

        let paragraph = tui::widgets::Paragraph::new(plot)
            .style(Style::default().fg(Color::White));
        
        paragraph.render(area, buf);
    }
}
