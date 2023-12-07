use super::{client::Client, id::PlaylistId, model};

pub async fn get_playlists_by_ids<'a, I>(
    client: Client,
    ids: I,
) -> crate::Result<Vec<model::PlaylistPartial>>
where
    I: IntoIterator<Item = &'a PlaylistId>,
{
    let mut playlists = vec![];

    for id in ids {
        playlists.push(client.playlist(id).await?);
    }

    Ok(playlists)
}
