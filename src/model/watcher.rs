use super::playlist::PlaylistType;

#[derive(Debug)]
pub struct Watcher {
    pub id: i64,
    pub user_id: String,
    pub playlist_from: PlaylistType,
    pub playlist_to: PlaylistType,
    pub should_remove: bool,
}

impl Watcher {
    pub fn try_from_row_data(
        id: i64,
        user_id: String,
        playlist_from: String,
        playlist_to: String,
        should_remove: bool,
    ) -> crate::Result<Self> {
        Ok(Self {
            id,
            user_id,
            playlist_from: PlaylistType::from_value(&playlist_from),
            playlist_to: PlaylistType::from_value(&playlist_to),
            should_remove,
        })
    }
}
