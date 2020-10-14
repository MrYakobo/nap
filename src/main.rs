use clap::{App, AppSettings, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use std::{process::exit, thread, time};

fn sleep(num_sec: f64) {
    //if passing infinity
    let num_millis = (num_sec * 1000.0) as u64;
    let t = time::Duration::from_millis(num_millis);
    let start = time::Instant::now();
    let one_frame = time::Duration::from_millis(12);

    let mut secs = t.as_secs();
    let hours = secs / 3600;
    secs %= 3600;
    let minutes = secs / 60;
    secs %= 60;

    let template_string = format!(
        "{} [{:02}:{:02}:{:02}]",
        "{spinner:.magenta.bold} [{elapsed_precise}] [{bar:60.cyan/blue}]", hours, minutes, secs
    );

    let bar = ProgressBar::new(num_millis);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(&template_string)
            .progress_chars("#>-"),
    );

    while start.elapsed() <= t {
        // set bar position to the remaining time
        let remaining_time = t - start.elapsed();
        bar.set_position(num_millis - remaining_time.as_millis() as u64);
        thread::sleep(one_frame);
    }
    bar.finish();
}

//parses the string and (possibly) returns the amount of seconds to sleep
//s: 1h, 2d, 33m, 100s, .1s
pub fn parse(s: &str) -> Option<f64> {
    let last_char = s.chars().last()?;
    let mut last_is_digit = false;

    let multiplier = match last_char {
        'd' => 60 * 60 * 24,
        'h' => 60 * 60,
        'm' => 60,
        's' => 1,
        '0'..='9' => {
            // default is seconds
            last_is_digit = true;
            1
        }
        _ => return None,
    };

    let possibly_num = if last_is_digit {
        &s
    } else {
        &s[0..s.len() - 1] // strip unit (d|h|m|s)
    };

    // parse as int, if that doesn't work parse as float
    let num: f64 = match possibly_num.parse::<u64>() {
        Ok(n) => n as f64,
        Err(_) => match possibly_num.parse::<f64>() {
            Ok(f) => f,
            Err(_) => return None,
        },
    };

    // apply modifier
    Some(num * (multiplier as f64))
}

fn main() {
    let matches = App::new("nap")
        .version("0.1.0")
        .about("Like coreutils sleep(1), but with progress")
        .arg(Arg::with_name("NUMBER[SUFFIX]").help("SUFFIX may be 's' for seconds (default), 'm' for minutes, 'h' for hours or 'd' for days. Given two or more arguments, nap for the sum of the values.").takes_value(true).multiple(true))
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let args = matches.values_of("NUMBER[SUFFIX]");

    match args {
        None => {
            eprintln!("Argument not given");
            exit(1);
        }
        Some(args) => {
            let mut sum = 0.0;
            let mut infinity = false;
            for s in args {
                if s == "infinity" {
                    infinity = true;
                    break;
                }
                let maybe_numsec = parse(s);
                match maybe_numsec {
                    Some(n) => {
                        sum += n;
                    }
                    None => {
                        eprintln!("Invalid argument \"{}\", could not be parsed", s);
                        exit(1);
                    }
                }
            }

            if infinity {
                let b = ProgressBar::new(0);
                b.set_style(ProgressStyle::default_bar().template("{spinner:.magenta.bold}"));
                let one_frame = time::Duration::from_millis(12);
                loop {
                    thread::sleep(one_frame);
                    b.inc(0);
                }
            } else {
                sleep(sum);
            }
        }
    }
}
