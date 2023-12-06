use super::{client::Client, model};

pub async fn get_playlists_by_ids<'a, I>(
    client: Client,
    ids: I,
) -> crate::Result<Vec<model::Playlist>>
where
    I: IntoIterator<Item = &'a String>,
{
    let mut playlists = vec![];

    for id in ids {
        playlists.push(client.get_playlist(id).await?);
    }

    Ok(playlists)
}
