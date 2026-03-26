#[derive(Debug)]
pub struct Period {
    pub name: String,
    pub day: String,
    pub is_morning: bool,
    pub start_time: String,
    pub end_time: String,
    pub class_room: String,
}

#[derive(Debug)]
pub struct Exam {
    pub course_name: String,
    pub date: String,
    pub class: String,
    pub course_num: String,
    pub time: String,
    pub room: String,
    pub examiner: String,
}
