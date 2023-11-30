const headers = {
  Accept: "application/json",
  "Content-Type": "application/json",
};

const deleteMessage =
  "Are you sure? This will delete your connected data and watchers. Your Spotify account will be untouched, and you can always reconnect later";

/** @param {string} message */
function setError(message) {
  document.querySelector("#errors-text").innerHTML = message;
  document.querySelector("#errors").classList.remove("hidden");
}

function clearErrors() {
  document.querySelector("#errors-text").innerHTML = null;
  document.querySelector("#errors").classList.add("hidden");
}

function refresh() {
  // Refresh window to avoid having to do state management. It's just not worth using a JS library for this lol
  window.location.reload();
}

function playlistUpdated(e) {
  const from = document.querySelector("#input-playlist-from").value;
  const to = document.querySelector("#input-playlist-to").value;
  const sync_interval = document.querySelector("#input-sync-interval").value;

  document.querySelector("#submit").disabled =
    !from || !to || !sync_interval || from === to;
}

async function deleteUser() {
  if (!confirm(deleteMessage)) return;

  clearErrors();

  const res = await fetch("/me", { method: "DELETE", headers });
  const data = await res.json();
  if (!data.success) return setError(data.error);

  window.location.href = "/";
}

/** @param {string} id */
async function deleteWatcher(id) {
  clearErrors();

  const res = await fetch(`/watchers/${id}`, { method: "DELETE", headers });
  const data = await res.json();
  if (!data.success) return setError(data.error);

  refresh();
}

/** @param {string} id */
async function syncWatcher(id) {
  clearErrors();

  const res = await fetch(`/watchers/${id}/sync`, { method: "POST", headers });
  const data = await res.json();
  if (!data.success) return setError(data.error);

  refresh();
}

document.querySelector("form#create").addEventListener(
  "submit",
  async function (e) {
    clearErrors();
    e.preventDefault();

    const playlist_from = document.querySelector("#input-playlist-from").value;
    const playlist_to = document.querySelector("#input-playlist-to").value;
    const should_remove = document.querySelector(
      "#checkbox-should-remove"
    ).checked;
    const sync_interval = document.querySelector("#input-sync-interval").value;

    const res = await fetch("/watchers", {
      method: "POST",
      headers,
      body: JSON.stringify({
        playlist_from,
        playlist_to,
        should_remove,
        sync_interval,
      }),
    });

    const data = await res.json();
    if (!data.success) return setError(data.error);

    refresh();
  },
  true
);
