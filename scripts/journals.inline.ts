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

function toggleJournals(this: HTMLElement, evt: MouseEvent) {
  // Don't toggle if clicking the title link
  const target = evt.target as HTMLElement
  if (target.closest("a")) return

  const journals = this.closest(".journals") as HTMLElement
  if (!journals) return

  const wasCollapsed = journals.classList.contains("collapsed")
  const isCollapsed = journals.classList.toggle("collapsed")
  this.setAttribute("aria-expanded", isCollapsed ? "false" : "true")

  // If expanding, collapse other sections
  if (wasCollapsed && !isCollapsed) {
    collapseOtherSections(".journals")
  }

  // Save state to localStorage
  localStorage.setItem("journals-collapsed", isCollapsed.toString())
}

function setupJournals() {
  const allJournals = document.querySelectorAll(".journals") as NodeListOf<HTMLElement>

  for (const journals of allJournals) {
    // Restore collapsed state from localStorage
    const savedState = localStorage.getItem("journals-collapsed")
    if (savedState === "true") {
      journals.classList.add("collapsed")
      const toggle = journals.querySelector(".journals-toggle")
      if (toggle) {
        toggle.setAttribute("aria-expanded", "false")
      }
    }

    // Set up toggle button
    const toggleButton = journals.querySelector(".journals-toggle")
    if (toggleButton) {
      toggleButton.addEventListener("click", toggleJournals)
      window.addCleanup(() => toggleButton.removeEventListener("click", toggleJournals))
    }
  }

  // Also intercept Explorer toggle to collapse others when it expands
  const explorerToggle = document.querySelector(".explorer-toggle.desktop-explorer")
  if (explorerToggle) {
    const handleExplorerClick = () => {
      const explorer = document.querySelector(".explorer") as HTMLElement
      // Check if explorer will be expanded (it's currently collapsed)
      if (explorer?.classList.contains("collapsed")) {
        // Explorer is about to expand, collapse others
        setTimeout(() => {
          if (!explorer.classList.contains("collapsed")) {
            collapseOtherSections(".explorer")
          }
        }, 10)
      }
    }
    explorerToggle.addEventListener("click", handleExplorerClick)
    window.addCleanup(() => explorerToggle.removeEventListener("click", handleExplorerClick))
  }
}

document.addEventListener("nav", () => {
  setupJournals()
})
