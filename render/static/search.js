// logseq-publish client-side search with fuzzy matching
(function () {
  "use strict";

  var searchInput = document.getElementById("search-input");
  var searchResults = document.getElementById(
    "search-results",
  );
  var searchIndex = null;

  if (!searchInput || !searchResults) return;

  fetch("/search-index.json")
    .then(function (r) {
      return r.json();
    })
    .then(function (data) {
      searchIndex = data;
    })
    .catch(function () {});

  // Fuzzy match: returns score (0 = no match, higher = better)
  // Matches characters of query in order within target, allows gaps
  function fuzzyMatch(query, target) {
    var q = query.toLowerCase();
    var t = target.toLowerCase();

    // Exact substring — best score
    var idx = t.indexOf(q);
    if (idx !== -1) {
      // Bonus for match at start, or at word boundary
      var base = 100 + q.length * 10;
      if (idx === 0) base += 50;
      else if (
        t[idx - 1] === " " ||
        t[idx - 1] === "/" ||
        t[idx - 1] === "-"
      )
        base += 30;
      return base;
    }

    // Word-prefix matching: each query word starts a word in target
    var qWords = q.split(/\s+/);
    if (qWords.length > 1) {
      var tLower = t;
      var allFound = true;
      for (var w = 0; w < qWords.length; w++) {
        var re = new RegExp(
          "(^|[\\s/\\-])" + escapeRegex(qWords[w]),
        );
        if (!re.test(tLower)) {
          allFound = false;
          break;
        }
      }
      if (allFound) return 80 + q.length * 5;
    }

    // Sequential character matching with penalty for gaps
    var qi = 0,
      ti = 0,
      score = 0,
      consecutive = 0,
      firstMatch = -1;
    while (qi < q.length && ti < t.length) {
      if (q[qi] === t[ti]) {
        if (firstMatch === -1) firstMatch = ti;
        consecutive++;
        score += consecutive * 2; // Reward consecutive matches
        // Bonus for word-boundary matches
        if (
          ti === 0 ||
          t[ti - 1] === " " ||
          t[ti - 1] === "/" ||
          t[ti - 1] === "-"
        )
          score += 5;
        qi++;
      } else {
        consecutive = 0;
      }
      ti++;
    }

    if (qi < q.length) return 0; // Not all query chars matched

    // Penalize long gaps (spread-out matches are worse)
    var spread = ti - firstMatch;
    var density = q.length / spread;
    score = score * density;

    return Math.max(score, 1);
  }

  function escapeRegex(s) {
    return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  }

  function scoreEntry(entry, query) {
    var titleScore = fuzzyMatch(query, entry.title) * 3; // Title matches weighted 3x
    var tagScore = 0;
    if (entry.tags) {
      for (var i = 0; i < entry.tags.length; i++) {
        var s = fuzzyMatch(query, entry.tags[i]) * 2; // Tags weighted 2x
        if (s > tagScore) tagScore = s;
      }
    }
    var excerptScore = fuzzyMatch(
      query,
      entry.excerpt || "",
    );
    var focusBoost = Math.sqrt(entry.focus || 0) * 50;
    return Math.max(titleScore, tagScore, excerptScore) + focusBoost;
  }

  var activeIndex = -1;

  function updateActive() {
    var items = searchResults.querySelectorAll("a");
    for (var i = 0; i < items.length; i++) {
      if (i === activeIndex) {
        items[i].classList.add("active");
        items[i].scrollIntoView({ block: "nearest" });
      } else {
        items[i].classList.remove("active");
      }
    }
  }

  searchInput.addEventListener("input", function () {
    var query = this.value.trim();
    activeIndex = -1;

    if (!query || !searchIndex) {
      searchResults.innerHTML = "";
      return;
    }

    var scored = [];
    for (var i = 0; i < searchIndex.length; i++) {
      var s = scoreEntry(searchIndex[i], query);
      if (s > 0)
        scored.push({ entry: searchIndex[i], score: s });
    }

    scored.sort(function (a, b) {
      return b.score - a.score;
    });
    var results = scored.slice(0, 12);

    if (results.length === 0) {
      searchResults.innerHTML =
        '<p style="opacity:0.5;padding:0.5em">No results found.</p>';
      return;
    }

    searchResults.innerHTML = results
      .map(function (r) {
        return (
          '<a href="' +
          r.entry.url +
          '">' +
          '<div class="search-result-title">' +
          escapeHtml(r.entry.title) +
          "</div>" +
          (r.entry.excerpt
            ? '<div class="search-result-excerpt">' +
              escapeHtml(
                r.entry.excerpt.substring(0, 120),
              ) +
              "</div>"
            : "") +
          "</a>"
        );
      })
      .join("");
  });

  searchInput.addEventListener("keydown", function (e) {
    var items = searchResults.querySelectorAll("a");
    if (!items.length) return;

    if (e.key === "ArrowDown") {
      e.preventDefault();
      activeIndex = (activeIndex + 1) % items.length;
      updateActive();
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      activeIndex =
        activeIndex <= 0
          ? items.length - 1
          : activeIndex - 1;
      updateActive();
    } else if (
      e.key === "Enter" &&
      activeIndex >= 0 &&
      items[activeIndex]
    ) {
      e.preventDefault();
      items[activeIndex].click();
    }
  });

  document.addEventListener("keydown", function (e) {
    if (
      e.key === "/" &&
      document.activeElement !== searchInput
    ) {
      e.preventDefault();
      searchInput.focus();
    }
    if (
      e.key === "Escape" &&
      document.activeElement === searchInput
    ) {
      searchInput.blur();
      searchResults.innerHTML = "";
      activeIndex = -1;
    }
  });

  function escapeHtml(str) {
    var div = document.createElement("div");
    div.textContent = str;
    return div.innerHTML;
  }
})();
