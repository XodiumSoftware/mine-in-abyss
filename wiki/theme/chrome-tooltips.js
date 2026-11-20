// Styled tooltips for the site chrome: the navbar icon buttons, the print and
// repository links, the suggest-an-edit link, and the previous/next chapter
// arrows.
//
// mdBook writes these as native `title` attributes, which the browser renders
// in its own unstyled popup. The title is moved onto a data attribute so the
// native popup never appears, and a pseudo-element draws the tooltip instead.
//
// The tooltip is centred on its host, but a host near the edge of the page
// would pull half the box outside it, where `html { overflow-x: clip }` cuts it
// off. Such a host is marked end- or start-aligned instead, which is decided
// from the host's position so it also follows an RTL sidebar and a resized
// window. The box is never measured: it is a pseudo-element, and its widest
// case is the 260px cap in navbar.css, so half of that is the margin to keep.

(() => {
  const SELECTORS = [
      ".left-buttons > *",
      ".right-buttons > *",
      ".nav-chapters",
      ".mobile-nav-chapters",
  ];

  const HOST_ATTR = "data-chrome-tooltip";
  const ALIGN_ATTR = "data-chrome-tooltip-align";
  const MAX_HALF_WIDTH = 130; // px, half of the tooltip's 260px cap
  const EDGE = 8; // px, keeps the frame clear of the page edge

  const align = () => {
      document.querySelectorAll(`.chrome-tooltip[${HOST_ATTR}]`).forEach((element) => {
          const box = element.getBoundingClientRect();
          const centre = box.left + box.width / 2;
          const overflowEnd = centre + MAX_HALF_WIDTH > window.innerWidth - EDGE;
          const overflowStart = centre - MAX_HALF_WIDTH < EDGE;

          if (overflowEnd) {
              element.setAttribute(ALIGN_ATTR, "end");
          } else if (overflowStart) {
              element.setAttribute(ALIGN_ATTR, "start");
          } else {
              element.removeAttribute(ALIGN_ATTR);
          }
      });
  };

  const adopt = () => {
      const elements = document.querySelectorAll(SELECTORS.join(","));

      elements.forEach((element) => {
          const label =
              element.getAttribute("title") || element.getAttribute("aria-label");
          if (!label) return;

          element.setAttribute(HOST_ATTR, label);
          element.removeAttribute("title");
          element.classList.add("chrome-tooltip");
      });

      align();
  };

  window.addEventListener("resize", align);

  if (document.readyState === "loading") {
      document.addEventListener("DOMContentLoaded", adopt);
  } else {
      adopt();
  }
})();
