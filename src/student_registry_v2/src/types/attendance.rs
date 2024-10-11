use serde::{Deserialize, Serialize};

// Attendance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceStruct {
    pub date: String,
    pub time_in: String,
    pub time_out: String,
    pub attendance_status: AttendanceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttendanceStatus {
    Present,
    Absent,
}
