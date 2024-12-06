use std::ops::Range;

use another_html_builder::{Body, Buffer};

fn format_hourly(ts: &u64) -> String {
    chrono::DateTime::from_timestamp(*ts as i64, 0)
        .map(|ts| ts.format("%H:%M").to_string())
        .unwrap_or_default()
}

#[derive(Debug)]
pub struct Serie<'a> {
    name: &'a str,
    point_size: u32,
    values: Vec<(u64, f64)>,
}

impl<'a> Serie<'a> {
    pub fn new(name: &'a str, values: Vec<(u64, f64)>) -> Self {
        Self {
            name,
            point_size: 1,
            values,
        }
    }

    pub fn with_point_size(mut self, value: u32) -> Self {
        self.point_size = value;
        self
    }
}

fn from_chart_error<E: std::error::Error + Send + Sync + 'static>(err: E) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Interrupted, err)
}

#[derive(Debug)]
pub struct LineChart<'a> {
    series: Vec<Serie<'a>>,
    size: (u32, u32),
    margin_left: u32,
    margin_bottom: u32,
    x_range: Option<Range<u64>>,
    y_range: Option<Range<f64>>,
}

impl<'a> LineChart<'a> {
    pub fn new(
        size: (u32, u32),
        margin_left: u32,
        margin_bottom: u32,
        series: Vec<Serie<'a>>,
        x_range: Option<Range<u64>>,
        y_range: Option<Range<f64>>,
    ) -> Self {
        Self {
            series,
            size,
            margin_left,
            margin_bottom,
            x_range,
            y_range,
        }
    }

    pub fn add_serie(&mut self, serie: Serie<'a>) {
        self.series.push(serie);
    }

    pub fn with_serie(mut self, serie: Serie<'a>) -> Self {
        self.series.push(serie);
        self
    }

    pub fn with_x_range(mut self, range: Range<u64>) -> Self {
        self.x_range = Some(range);
        self
    }

    pub fn set_x_range(&mut self, range: Option<Range<u64>>) {
        self.x_range = range;
    }

    pub fn with_y_range(mut self, range: Range<f64>) -> Self {
        self.y_range = Some(range);
        self
    }

    pub fn set_y_range(&mut self, range: Option<Range<f64>>) {
        self.y_range = range;
    }

    fn x_range(&self) -> Option<Range<u64>> {
        if let Some(ref value) = self.x_range {
            Some(value.clone())
        } else {
            self.series
                .iter()
                .flat_map(|s| s.values.iter().map(|(ts, _)| *ts))
                .fold(None::<(u64, u64)>, |prev, value| match prev {
                    Some((min, max)) => Some((min.min(value), max.max(value))),
                    None => Some((value, value)),
                })
                .map(|(min, max)| min..max)
        }
    }

    fn y_range(&self) -> Option<Range<f64>> {
        if let Some(ref value) = self.y_range {
            Some(value.clone())
        } else {
            self.series
                .iter()
                .flat_map(|s| s.values.iter().map(|(_, value)| *value))
                .fold(None::<(f64, f64)>, |prev, value| match prev {
                    Some((min, max)) => Some((min.min(value), max.max(value))),
                    None => Some((value, value)),
                })
                .map(|(min, max)| min..max)
        }
    }

    fn into_svg(&self) -> Result<String, std::io::Error> {
        use plotters::prelude::*;

        let Some(x_range) = self.x_range() else {
            return Ok(String::default());
        };
        let Some(y_range) = self.y_range() else {
            return Ok(String::default());
        };
        // TODO find a way to access the buffer content
        let mut buffer = String::new();
        {
            let root = plotters::backend::SVGBackend::with_string(&mut buffer, self.size)
                .into_drawing_area();
            root.fill(&WHITE).map_err(from_chart_error)?;
            let mut chart = ChartBuilder::on(&root)
                .margin(10)
                .set_label_area_size(LabelAreaPosition::Left, self.margin_left)
                .set_label_area_size(LabelAreaPosition::Bottom, self.margin_bottom)
                .build_cartesian_2d(x_range, y_range)
                .map_err(from_chart_error)?;

            chart
                .configure_mesh()
                .disable_x_mesh()
                .disable_y_mesh()
                // .x_labels(30)
                // .max_light_lines(4)
                .x_label_formatter(&format_hourly)
                .draw()
                .map_err(from_chart_error)?;

            for serie in self.series.iter() {
                chart
                    .draw_series(
                        LineSeries::new(serie.values.iter().copied(), &RED)
                            .point_size(serie.point_size),
                    )
                    .map_err(from_chart_error)?
                    .label(serie.name);
            }
        }

        Ok(buffer)
    }
}

impl<'a> crate::component::prelude::Component for LineChart<'a> {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        match self.into_svg() {
            Ok(svg) => buf.raw(svg),
            Err(err) => {
                tracing::warn!(message = "unable to generate svg", error = %err);
                buf
            }
        }
    }
}
