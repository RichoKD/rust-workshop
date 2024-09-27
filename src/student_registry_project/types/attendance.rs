// Attendance
#[derive(Debug, Clone)]
pub struct AttendanceStruct {
    pub date: String,
    pub time_in: String,
    pub time_out: String,
    pub attendance_status: AttendanceStatus,
}

#[derive(Debug, Clone)]
pub enum AttendanceStatus {
    Present,
    Absent,
}
