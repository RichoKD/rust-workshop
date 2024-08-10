#[derive(Debug, Clone)]
pub struct Student {
    pub first_name: String,
    pub last_name: String,
    pub id: u32,
    pub age: u8,
    pub height: f32,
    pub sex: Sex,
}

#[derive(Debug, Clone)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Debug, Clone)]
pub struct StudentRegistry {
    pub total_students: Vec<Student>,
}

/***
 *
 * [
 *  {1},
 *  {2},
 *  {3},
 * ]
 */
