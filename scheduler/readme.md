**Rust Scheduler with Task Execution**  
This Rust application demonstrates the usage of schedulers for task execution at specified intervals.   
**Overview**  
The application utilizes the clokwerk and job_scheduler crates to schedule tasks based on various time intervals and cron timings. It   includes examples of running tasks periodically, daily, biweekly, and at specific times.
  
**Features**  
**Periodic Task:** Execute a task every second and every minute.  
**Daily Task:** Run a task daily at 6:44 PM.  
**Biweekly Task:** Perform a task on Tuesdays at 2:20:17 PM and on Thursdays at 3:00 PM.  
**Send Request:** Send an HTTP request every 2 seconds.  
**Cron Timing Scheduling:** Schedule a task to execute every year on April 29th at 09:45 AM in UTC.formate will be like this and default   timing will be in UTC format  
# sec   min   hour   day of month   month   day of week   year  
#  *     *     *      *              *       *             *  
**Getting Started**  
Clone the repository to your local machine.  
Ensure you have Rust installed.  
Set up your environment variables in a .env file with the required configurations, such as the URL for sending requests and cron timings.  
Run the application using cargo run.  
**Usage**  
Modify the code to suit your task execution requirements.  
Adjust the time intervals, cron timings, and task actions as needed.  
Run the application and observe the task execution at the specified intervals.  
**Dependencies**  
clokwerk: A crate for scheduling tasks based on time intervals.  
job_scheduler: A crate for scheduling tasks using cron timing expressions.  
reqwest: A crate for making HTTP requests.  
dotenv: A crate for loading environment variables from a .env file.  
**Contributions**  
Contributions to improve the functionality or add new features are welcome. Please follow the standard Rust guidelines and ensure that   tests are included for any new functionality.
  
