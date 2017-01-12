#[derive(Debug)]
pub struct RoomKey {
    pub encrypted_name: String,
    pub sector_id: u32,
    pub check_sum: String,
}