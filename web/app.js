/**
 * Page bootstrap. No animation: the heading and loading copy stay static.
 * @returns {void}
 */
export function boot() {
  const state = document.getElementById("state");
  if (state) {
    state.textContent = "Loading the Colony shell.";
  }
}

boot();
