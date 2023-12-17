use reqwest::StatusCode;

use super::{
    client::{Client, WithToken},
    error::ClientResult,
    id::{PlaylistId, UserId},
    model,
};

/// Fetch all playlist partials from a list of playlist IDs
pub async fn get_playlists_by_ids<'a, I>(
    client: &Client<WithToken>,
    ids: I,
) -> ClientResult<Vec<model::PlaylistPartial>>
where
    I: IntoIterator<Item = &'a PlaylistId>,
{
    let mut playlists = vec![];

    for id in ids {
        playlists.push(client.playlist_partial(id).await?);
    }

    Ok(playlists)
}

/// Check if a playlist is editable by the current user
pub async fn check_playlist_editable(
    client: &Client<WithToken>,
    id: &PlaylistId,
    user_id: &UserId,
) -> ClientResult<bool> {
    // Get playlist details
    let playlist = client.playlist_partial(&id).await?;

    // If the user owns the playlist, don't try making request
    if playlist.owner.id == *user_id {
        return Ok(true);
    }

    // Attempt to update name to itself to check permissions
    let res = client.playlist_update_name(&id, &playlist.name).await;

    Ok(match res {
        Ok(_) => true,
        Err(ref _err @ super::error::ClientError::ApiError { status, message: _ })
            if status == StatusCode::FORBIDDEN.as_u16() =>
        {
            false
        }
        Err(err) => return Err(err),
    })
}
