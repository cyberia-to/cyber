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

// Override the pushState method for correct hashchange event (for analytics)
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

// Custom sidebar menu
(function () {
  var menu = [
    { icon: "🤖", name: "Cyb", page: "cyb" },
    { icon: "🔵", name: "Cyber", page: "cyber" },
    { icon: "🌏", name: "Cyberia", page: "cyberia" },
    {
      icon: "⛰",
      name: "Cyber Valley",
      page: "cyber valley",
    },
    { icon: "🟢", name: "Bostrom", page: "bostrom" },
    {
      icon: "🔴",
      name: "Cybernomics",
      page: "cybernomics",
    },
    { icon: "🌀", name: "Cybics", page: "cybics" },
    { icon: "🧠", name: "Superhuman", page: "superhuman" },
  ];

  var permanent = [
    { icon: "📁", name: "Pages", hash: "#/all-pages" },
    { icon: "📦", name: "Topics", hash: "#/all-pages" },
    { icon: "🕸", name: "Graph", hash: "#/graph" },
    { icon: "📝", name: "Blog", hash: "#/all-journals" },
  ];

  function currentPage() {
    var h = window.location.hash || "";
    var m = h.match(/^#\/page\/(.+)/);
    return m ? decodeURIComponent(m[1]).toLowerCase() : "";
  }

  function buildMenu() {
    var cur = currentPage();
    var nav = document.getElementById("cyber-nav");
    if (!nav) {
      nav = document.createElement("nav");
      nav.id = "cyber-nav";
    }
    nav.innerHTML = "";

    var ul = document.createElement("ul");

    menu.forEach(function (item) {
      var li = document.createElement("li");
      var a = document.createElement("a");
      a.href = "#/page/" + encodeURIComponent(item.page);
      if (cur === item.page.toLowerCase())
        a.classList.add("active");
      a.innerHTML =
        '<span class="cyber-nav-icon">' +
        item.icon +
        "</span>" +
        '<span class="cyber-nav-label">' +
        item.name +
        "</span>";
      li.appendChild(a);
      ul.appendChild(li);
    });

    // separator
    var sep = document.createElement("li");
    sep.className = "cyber-nav-sep";
    ul.appendChild(sep);

    permanent.forEach(function (item) {
      var li = document.createElement("li");
      var a = document.createElement("a");
      a.href = item.hash;
      a.innerHTML =
        '<span class="cyber-nav-icon">' +
        item.icon +
        "</span>" +
        '<span class="cyber-nav-label">' +
        item.name +
        "</span>";
      li.appendChild(a);
      ul.appendChild(li);
    });

    nav.appendChild(ul);
    return nav;
  }

  function inject() {
    var sidebar = document.querySelector(
      ".left-sidebar-inner",
    );
    if (!sidebar) return false;

    // Hide Logseq native favorites and nav
    var groups = sidebar.querySelectorAll(
      ".sidebar-content-group",
    );
    groups.forEach(function (g) {
      g.style.display = "none";
    });
    var navs = sidebar.querySelectorAll(
      ".sidebar-navigations",
    );
    navs.forEach(function (n) {
      n.style.display = "none";
    });

    // Hide the graph/user header section but keep the sidebar container
    var header = sidebar.querySelector(
      ".sidebar-header-container",
    );
    if (header) header.style.display = "none";

    var nav = buildMenu();
    if (!document.getElementById("cyber-nav")) {
      sidebar.prepend(nav);
    }
    return true;
  }

  // Poll until sidebar DOM exists
  var attempts = 0;
  var interval = setInterval(function () {
    if (inject() || ++attempts > 30)
      clearInterval(interval);
  }, 500);

  // Rebuild on navigation to update active state
  window.addEventListener("hashchange", function () {
    setTimeout(function () {
      var nav = document.getElementById("cyber-nav");
      if (nav) {
        var fresh = buildMenu();
        nav.replaceWith(fresh);
      }
    }, 100);
  });
})();
