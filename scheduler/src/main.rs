// Scheduler and trait for defining time intervals
use chrono::{self};
use clokwerk::Interval::*;
use clokwerk::{Job, Scheduler, TimeUnits};
use dotenv::dotenv;
use job_scheduler::{Job as js, JobScheduler};
use std::time::Duration;
use std::{env, thread};

fn main() {
    // Create a new scheduler
    dotenv().ok();
    let mut scheduler = Scheduler::with_tz(chrono::Local);

    // Add some tasks to it
    scheduler
        .every(1.second())
        .run(|| println!("Periodic task"));
    scheduler
        .every(1.minute())
        .run(|| println!("Every 60 seconds"));
    scheduler
        .every(1.day())
        .at("6:44 pm")
        .run(|| println!("Daily task"));
    scheduler
        .every(Tuesday)
        .at("14:20:17")
        .and_every(Thursday)
        .at("15:00")
        .run(|| println!("Biweekly task"));
    scheduler.every(2.seconds()).run(send_request); // Send request every 2 seconds

    // Manually run the scheduler in an event loop
    for _ in 1..100 {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(1000));
    }

    // Or run it in a background thread
    let thread_handle = scheduler.watch_thread(Duration::from_millis(100));
    // The scheduler stops when `thread_handle` is dropped, or `stop` is called
    thread_handle.stop();

    // Using cron timing scheduling
    let mut sched = JobScheduler::new();
    let cron_time = fetch_var_from_env("CRON_TIMING");
    println!("Cron time from env: {}", cron_time);
    sched.add(js::new(cron_time.parse().unwrap(), || {
        println!("I get executed every year on April 29th at 09:45 AM in UTC");
    }));

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn send_request() {
    // Create a reqwest Client
    let client = reqwest::Client::new();
    let url = fetch_var_from_env("URL"); // Specify the URL to send the request to

    // Send the GET request and await the response
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let response = match client.get(&url).send().await {
            Ok(val) => val,
            Err(e) => panic!("Error in sending request: {}", e),
        };

        // Check if the request was successful (status code 200)
        if response.status().is_success() {
            // Read the response body as a string
            let _body = match response.text().await {
                Ok(val) => val,
                Err(e) => panic!("Error: {}", e),
            };
            // println!("Response body: {}", body);
        } else {
            println!("Request failed with status code: {}", response.status());
        }
    });
}

fn fetch_var_from_env(var_name: &str) -> String {
    match env::var(var_name) {
        Ok(val) => val,
        Err(e) => panic!("Error: {} {}", var_name, e),
    }
}
