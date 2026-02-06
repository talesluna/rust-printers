use std::time::SystemTime;

use printers::*;

#[test]
fn test_get_job_history() {
    for printer in get_printers() {
        for job in printer.get_job_history() {
            assert!(job.id > 0);
            assert!(job.created_at > SystemTime::UNIX_EPOCH);
        }
    }
}
