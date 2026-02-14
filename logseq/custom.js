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

// Build sidebar menu from pages tagged with menu
(function buildMenuFromTags() {
  // Query pages with tags:: menu using datascript
  function queryMenuPages() {
    try {
      if (!logseq || !logseq.api) return null;
      var results = logseq.api.datascript_query(
        '[:find (pull ?p [:block/name :block/properties :block/original-name]) :where [?t :block/name "menu"] [?p :block/tags ?t]]',
      );
      if (!results || !results.length) return null;
      return results.map(function (r) {
        return r[0];
      });
    } catch (e) {
      return null;
    }
  }

  function getPageIcon(props) {
    if (!props) return "";
    return props.icon || "";
  }

  function getPageTitle(page) {
    var name = page["original-name"] || page.name || "";
    return name.charAt(0).toUpperCase() + name.slice(1);
  }

  function buildFavoriteItem(page) {
    var li = document.createElement("li");
    li.className = "favorite-item";
    var a = document.createElement("a");
    var pageName = page.name || page["original-name"] || "";
    a.href = "#/page/" + encodeURIComponent(pageName);
    a.className = "item-container";
    a.dataset.ref = pageName;

    var icon = getPageIcon(page.properties);
    var title = getPageTitle(page);

    var span = document.createElement("span");
    span.className = "page-icon";
    span.textContent = icon;

    var textSpan = document.createElement("span");
    textSpan.className = "page-title flex-1";
    textSpan.textContent = title;

    if (icon) {
      a.appendChild(span);
      a.appendChild(document.createTextNode(" "));
    }
    a.appendChild(textSpan);
    li.appendChild(a);
    return li;
  }

  function rebuildFavorites(pages) {
    // Find the favorites section
    var favGroups = document.querySelectorAll(
      ".sidebar-content-group",
    );
    var favGroup = null;
    for (var i = 0; i < favGroups.length; i++) {
      var hd = favGroups[i].querySelector(
        ".hd span, .hd .header",
      );
      if (
        hd &&
        hd.textContent &&
        hd.textContent.trim().toLowerCase() === "favorites"
      ) {
        favGroup = favGroups[i];
        break;
      }
    }
    if (!favGroup) return false;

    var bd = favGroup.querySelector(".bd ul");
    if (!bd) {
      bd = favGroup.querySelector(".bd");
      if (!bd) return false;
      var ul = document.createElement("ul");
      bd.appendChild(ul);
      bd = ul;
    }

    // Clear existing favorites
    bd.innerHTML = "";

    // Add menu-tagged pages
    pages.forEach(function (page) {
      bd.appendChild(buildFavoriteItem(page));
    });

    return true;
  }

  function renameJournalsToBlog() {
    var el = document.querySelector(
      ".journals-nav .flex-1",
    );
    if (el && el.innerHTML !== "Blog") {
      el.innerHTML = "Blog";
    }
  }

  // Poll until logseq.api is available and favorites DOM exists
  var attempts = 0;
  var maxAttempts = 60;
  var interval = setInterval(function () {
    attempts++;
    renameJournalsToBlog();

    var pages = queryMenuPages();
    if (pages && pages.length > 0) {
      // Sort by original name for consistent order
      pages.sort(function (a, b) {
        var na = (
          a["original-name"] ||
          a.name ||
          ""
        ).toLowerCase();
        var nb = (
          b["original-name"] ||
          b.name ||
          ""
        ).toLowerCase();
        return na.localeCompare(nb);
      });
      if (rebuildFavorites(pages)) {
        clearInterval(interval);
      }
    }
    if (attempts >= maxAttempts) {
      clearInterval(interval);
    }
  }, 1000);

  // Also rebuild on navigation
  window.addEventListener("hashchange", function () {
    setTimeout(function () {
      renameJournalsToBlog();
      var pages = queryMenuPages();
      if (pages && pages.length > 0) {
        pages.sort(function (a, b) {
          var na = (
            a["original-name"] ||
            a.name ||
            ""
          ).toLowerCase();
          var nb = (
            b["original-name"] ||
            b.name ||
            ""
          ).toLowerCase();
          return na.localeCompare(nb);
        });
        rebuildFavorites(pages);
      }
    }, 500);
  });
})();
