#[derive(Debug)]
pub struct DriveEntry {
    pub name: String,
    pub offset: Option<i16>,
    pub submission_count: Option<i32>,
    pub percentage_agree: Option<f64>,
}

impl DriveEntry {
    pub fn new(name: String, offset: Option<i16>, submission_count: Option<i32>, percentage_agree: Option<f64>) -> Self {
        DriveEntry {
            name,
            offset,
            submission_count,
            percentage_agree,
        }
    }
}