// change theme to black
if (document.querySelector("html").getAttribute("data-theme") === "light") {
  document.querySelector("html").setAttribute("data-theme", "dark");

  document.querySelector("body").classList.remove("light-theme");
  document.querySelector("body").classList.remove("white-theme");
  document.querySelector("body").classList.add("dark-theme");

  localStorage.setItem("theme", '"dark"');
}

document.querySelector(".journals-nav .flex-1").innerHTML = "Blog";

// Override the pushState method for correct hashchange event (for analytics)
// seems router issue
const originalPushState = history.pushState;

history.pushState = function (state, title, url) {
  originalPushState.apply(history, arguments);
  window.dispatchEvent(new Event("hashchange"));
};
