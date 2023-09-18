use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use sysinfo::{CpuExt, System, SystemExt};
struct SystemData {
    ram_usage: f32,
    cpu_usage: f32,
}
struct App {
    data: SystemData,
    system: System,
    ram_data: Vec<(f64, f64)>, // Historical RAM usage data
    cpu_data: Vec<(f64, f64)>, // Historical CPU usage data
    window: [f64; 2],          // Adjust the window bounds as needed
}

fn update_custom_data(sys: &mut System, data: &mut SystemData) {
    sys.refresh_all();
    let total_memory = sys.total_memory() as f32;
    let used_memory = sys.used_memory() as f32;
    sys.refresh_cpu();
    let mut total_percent: f32 = 0.0;
    let mut total_cpus: f32 = 0.0;
    for cpu in sys.cpus() {
        total_percent = total_percent + cpu.cpu_usage();
        total_cpus = total_cpus + 1.0;
    }
    data.ram_usage = (used_memory / total_memory) * 100.0;
    data.cpu_usage = total_percent / total_cpus;
}

impl App {
    fn new() -> App {
        let mut data = SystemData {
            ram_usage: 0.0,
            cpu_usage: 0.0,
        };

        let mut sys = System::new_all();
        // Initial update of custom data
        update_custom_data(&mut sys, &mut data);

        App {
            data,
            system: sys,
            ram_data: vec![(0.0, 0.0)], // Initial RAM data point
            cpu_data: vec![(0.0, 0.0)], // Initial CPU data point
            window: [0.0, 100.0],       // Adjust the window bounds as needed
        }
    }

    fn on_tick(&mut self) {
        // Update custom data
        update_custom_data(&mut self.system, &mut self.data);

        // Append new data points to historical data
        self.ram_data
            .push((self.window[1], self.data.ram_usage as f64));
        self.cpu_data
            .push((self.window[1], self.data.cpu_usage as f64));

        if self.ram_data.len() > 100 {
            self.ram_data.remove(0);
        }

        if self.cpu_data.len() > 100 {
            self.cpu_data.remove(0);
        }

        // Update the window range
        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let sysinfo_update_rate = Duration::from_millis(10); // Update sysinfo data every second
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            update_custom_data(&mut app.system, &mut app.data); // Update sysinfo data here
            last_tick = Instant::now();
        }

        // Add a condition to update sysinfo data separately
        if last_tick.elapsed() >= sysinfo_update_rate {
            update_custom_data(&mut app.system, &mut app.data); // Update sysinfo data here
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let x_labels = vec![
        Span::styled(
            format!("{}", app.window[0]),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("{}", (app.window[0] + app.window[1]) / 2.0)),
        Span::styled(
            format!("{}", app.window[1]),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ];

    let datasets = vec![
        Dataset::default()
            .name("RAM")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .graph_type(GraphType::Line)
            .data(&app.ram_data),
        Dataset::default()
            .name("CPU")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .graph_type(GraphType::Line)
            .data(&app.cpu_data),
    ];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title("System Usage".cyan().bold())
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("per second")
                .style(Style::default().fg(Color::Gray))
                .labels(x_labels)
                .bounds(app.window),
        )
        .y_axis(
            Axis::default()
                .title("Usage %")
                .style(Style::default().fg(Color::Gray))
                .labels(vec!["0".bold(), "0".into(), "100".bold()])
                .bounds([0.0, 100.0]), // Adjust the bounds as needed
        );
    f.render_widget(chart, size);
}
