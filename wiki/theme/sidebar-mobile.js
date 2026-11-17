// Starts a narrow viewport with the sidebar closed, and keeps the sidebar at a
// fixed width.
//
// mdBook opens the sidebar before it is drawn whenever the document is at least
// 1080px wide, but that test is horizontal only: a phone or a small laptop left
// with a short viewport clears it and gets a sidebar that covers most of the page
// on first load. It also only ever opens by itself, so a sidebar closed on a wide
// window stays closed on a phone once the saved choice is remembered.
//
// This script reads the same saved `mdbook-sidebar` entry and closes the sidebar
// again when the viewport is short or narrow, leaving the saved preference alone.
// Every page reloads this script, so the choice made on one page carries to the
// next without writing anything an mdBook version might disagree with.
//
// The sidebar is not resizable: navbar.css hides the resize handle and fixes
// --sidebar-target-width, and the handle is removed here so mdBook's resize code
// finds nothing to attach to.

(() => {
  const MIN_HEIGHT = 700; // px
  const MIN_WIDTH = 1080; // px, mdBook's own test

  const isCramped = () =>
      window.innerHeight < MIN_HEIGHT || window.innerWidth < MIN_WIDTH;

  const close = () => {
      const html = document.documentElement;
      if (!html.classList.contains("sidebar-visible")) return;

      html.classList.remove("sidebar-visible");

      const anchor = document.getElementById("mdbook-sidebar-toggle-anchor");
      if (anchor) anchor.checked = false;

      const toggle = document.getElementById("mdbook-sidebar-toggle");
      if (toggle) toggle.setAttribute("aria-expanded", "false");

      const sidebar = document.getElementById("mdbook-sidebar");
      if (sidebar) sidebar.setAttribute("aria-hidden", "true");

      document.querySelectorAll("#mdbook-sidebar a").forEach((link) => {
          link.setAttribute("tabIndex", -1);
      });
  };

  const init = () => {
      document.getElementById("mdbook-sidebar-resize-handle")?.remove();

      if (isCramped()) close();
  };

  if (document.readyState === "loading") {
      document.addEventListener("DOMContentLoaded", init);
  } else {
      init();
  }
})();
