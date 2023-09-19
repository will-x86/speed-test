use plotters::prelude::*;
use std::env;
use std::fs::File;
use std::io;
use std::process::Command;

use std::time::{Duration, Instant};
use sysinfo::{CpuExt, System, SystemExt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a system monitor
    let mut sys = System::new_all();

    // Initialize plotter
    let root = BitMapBackend::new("cpu_ram_usage.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create a chart
    let start_time = Instant::now();
    // Add CPU and RAM series
    let finish_at = Instant::now() + Duration::new(20, 0);
    let mut x_values = Vec::new();
    let mut cpu_values = Vec::new();
    let mut ram_values = Vec::new();
    let total_ram = sys.total_memory() as f32;
    let mut finished = false;
    let node_script_dir = "/home/w/go/src/github.com/seal/speed-test/node/";

    // Change the current working directory to the Node.js script directory
    env::set_current_dir(node_script_dir)?;
    // Start the Node.js program
    let mut server_process = Command::new("node")
        .arg("index.js") // Replace with your Node.js script
        .spawn()?;

    println!("Press Enter to start recording and start performance benchmark...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let log = File::create("./wrk.log").expect("failed to open log");
    let mut wrk_command = Command::new("wrk")
        .args(&["-t12", "-c400", "-d20s"])
        .args(&["http://127.0.0.1:3000/?q1=1&q2=2&q3=3&q4=4"])
        .stdout(log)
        .spawn()?;
    // Wait for the wrk command to finish
    while !finished {
        let used_memory = sys.used_memory() as f32;
        let mut total_percent: f32 = 0.0;
        let mut total_cpus: f32 = 0.0;
        sys.refresh_memory();
        sys.refresh_cpu();
        for cpu in sys.cpus() {
            total_percent = total_percent + cpu.cpu_usage();
            total_cpus = total_cpus + 1.0;
        }
        let elapsed_time = start_time.elapsed().as_secs(); // Calculate elapsed time in seconds
        x_values.push(elapsed_time);
        cpu_values.push((total_percent / total_cpus) as u64);
        ram_values.push((used_memory / total_ram * 100.0) as u64);
        std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
        if Instant::now() >= finish_at {
            finished = true;
        }
        println!(
            " pushed CPU {} and ram {}",
            total_percent / total_cpus,
            (used_memory / total_ram) * 100.0
        )
    }

    //let wrk_status = wrk_command.wait()?;
    // Stop the Node.js program (you may need to adjust this based on your specific Node.js script)
    //wrk_command.kill()?;
    wrk_command.wait().expect("failed to finish echo");

    server_process.kill()?;

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0u64..start_time.elapsed().as_secs(), 0u64..100u64)?;

    chart
        .configure_mesh()
        .x_desc("Time (s)")
        .y_desc("Usage (%)")
        .draw()?;

    chart.draw_series(LineSeries::new(
        x_values
            .iter()
            .zip(cpu_values.iter())
            .map(|(&x, &y)| (x, y)),
        &RED,
    ))?;

    chart.draw_series(LineSeries::new(
        x_values
            .iter()
            .zip(ram_values.iter())
            .map(|(&x, &y)| (x, y)),
        &BLUE,
    ))?;

    // Save the plot to a file
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}
