use super::playlist::PlaylistType;

pub struct Watcher {
    pub id: i64,
    pub user_id: String,
    pub user_token: rspotify::Token,
    pub from_playlist: PlaylistType,
    pub to_playlist: PlaylistType,
    pub should_remove: bool,
}

impl Watcher {
    pub fn try_from_row_data(
        id: i64,
        user_id: String,
        user_token: String,
        from_playlist: String,
        to_playlist: String,
        should_remove: bool,
    ) -> crate::Result<Self> {
        Ok(Self {
            id,
            user_id,
            user_token: serde_json::from_str(&user_token)?,
            from_playlist: PlaylistType::from_value(&from_playlist),
            to_playlist: PlaylistType::from_value(&to_playlist),
            should_remove,
        })
    }
}
