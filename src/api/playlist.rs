use rspotify::{
    clients::BaseClient,
    model::{FullPlaylist, PlaylistId},
    AuthCodeSpotify,
};

pub async fn get_playlists_by_ids<'a, I>(
    client: AuthCodeSpotify,
    ids: I,
) -> crate::Result<Vec<FullPlaylist>>
where
    I: IntoIterator<Item = &'a String>,
{
    let mut playlists = vec![];

    for id in ids {
        let id = PlaylistId::from_id_or_uri(id)?;
        playlists.push(client.playlist(id, None, None).await?);
    }

    Ok(playlists)
}
