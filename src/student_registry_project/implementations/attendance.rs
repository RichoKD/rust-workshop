use crate::student_registry_project::{
    types::attendance::{AttendanceStatus, AttendanceStruct},
    utils::{get_current_date, get_current_time},
};

impl AttendanceStruct {
    pub fn new() -> Self {
        AttendanceStruct {
            attendance_status: AttendanceStatus::Present,
            date: get_current_date(),
            time_in: get_current_time(),
            time_out: String::new(),
        }
    }
}
