use tc_models::clock::TimeFormat;

pub struct ClockState {
    pub clock_face_idx: u16,
    pub clock_time_fmt: TimeFormat,
}
