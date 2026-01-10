function collapseOtherSections(exceptClass: string) {
  const sections = [".journals", ".favorites", ".explorer"]
  for (const selector of sections) {
    if (selector === exceptClass) continue
    const section = document.querySelector(selector) as HTMLElement
    if (section && !section.classList.contains("collapsed")) {
      section.classList.add("collapsed")
      const toggle = section.querySelector(`${selector}-toggle, .explorer-toggle.desktop-explorer`)
      if (toggle) {
        toggle.setAttribute("aria-expanded", "false")
      }
      // Save state
      const key = selector.replace(".", "") + "-collapsed"
      localStorage.setItem(key, "true")
    }
  }
}

function toggleFavorites(this: HTMLElement) {
  const favorites = this.closest(".favorites") as HTMLElement
  if (!favorites) return

  const wasCollapsed = favorites.classList.contains("collapsed")
  const isCollapsed = favorites.classList.toggle("collapsed")
  this.setAttribute("aria-expanded", isCollapsed ? "false" : "true")

  // If expanding, collapse other sections
  if (wasCollapsed && !isCollapsed) {
    collapseOtherSections(".favorites")
  }

  // Save state to localStorage
  localStorage.setItem("favorites-collapsed", isCollapsed.toString())
}

function setupFavorites() {
  const allFavorites = document.querySelectorAll(".favorites") as NodeListOf<HTMLElement>

  for (const favorites of allFavorites) {
    // Restore collapsed state from localStorage
    const savedState = localStorage.getItem("favorites-collapsed")
    if (savedState === "true") {
      favorites.classList.add("collapsed")
      const toggle = favorites.querySelector(".favorites-toggle")
      if (toggle) {
        toggle.setAttribute("aria-expanded", "false")
      }
    }

    // Set up toggle button
    const toggleButton = favorites.querySelector(".favorites-toggle")
    if (toggleButton) {
      toggleButton.addEventListener("click", toggleFavorites)
      window.addCleanup(() => toggleButton.removeEventListener("click", toggleFavorites))
    }
  }
}

document.addEventListener("nav", () => {
  setupFavorites()
})
