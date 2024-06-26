pub mod consts {
    pub const CRON_EXPRESSION_15_SEC: &str = "0/15 * * * * *";
    pub const CRON_EXPRESSION_2_MIN: &str = "0 */2 * * * *";
    pub const CRON_EXPRESSION_5_MIN: &str = "0 */5 * * * *";
}
pub mod cron_util {
    use std::str::FromStr;
    use std::thread;
    use chrono::{TimeZone, Utc};
    use chrono_tz::Asia::Tokyo;
    use cron::Schedule;
    pub fn create_cronjob_with_schedule(cron_expression: &str, task: fn()) {
        let schedule = Schedule::from_str(cron_expression)
            .expect("Couldn't start the scheduler. Check the cron expression.");
        loop {
            let utc_now = Utc::now().naive_utc();
            let jst_now = Tokyo.from_utc_datetime(&utc_now);
            if let Some(next) = schedule.upcoming(jst_now.timezone()).take(1).next() {
                let until_next = next - jst_now;
                thread::sleep(until_next.to_std().unwrap());
                println!("Running task at {}", jst_now.to_string());
                task();
            }
        }
    }
}