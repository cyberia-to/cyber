// change theme to black
if (
  document
    .querySelector("html")
    .getAttribute("data-theme") === "light"
) {
  document
    .querySelector("html")
    .setAttribute("data-theme", "dark");

  document
    .querySelector("body")
    .classList.remove("light-theme");
  document
    .querySelector("body")
    .classList.remove("white-theme");
  document
    .querySelector("body")
    .classList.add("dark-theme");

  localStorage.setItem("theme", '"dark"');
}

document.querySelector(".journals-nav .flex-1").innerHTML =
  "Blog";

// Override the pushState method for correct hashchange event (for analytics)
// seems router issue
const originalPushState = history.pushState;

history.pushState = function (state, title, url) {
  originalPushState.apply(history, arguments);
  window.dispatchEvent(new Event("hashchange"));
};

// Redirect /tags/X to /page/X so tags navigate to the associated page
function redirectTagsToPage() {
  var hash = window.location.hash;
  if (hash.startsWith("#/tags/")) {
    var tag = hash.slice("#/tags/".length);
    window.location.hash = "#/page/" + tag;
  }
}

window.addEventListener("hashchange", redirectTagsToPage);
redirectTagsToPage();
