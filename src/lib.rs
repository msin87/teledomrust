// pub enum SmellStrength {
//     None = 0,
//     Light = 1,
//     Hard = 2,
// }

pub struct AirSmellData<'a> {
    pub id: i64,
    pub userid: i64,
    pub stinkvalue: u8,
    pub latitude: f32,
    pub longitude: f64,
    pub comment: &'a str,
    pub dir0: i16,
    pub dir1: i16,

}
