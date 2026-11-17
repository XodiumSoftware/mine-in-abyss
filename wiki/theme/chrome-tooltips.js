// Styled tooltips for the site chrome: the navbar icon buttons, the print and
// repository links, the suggest-an-edit link, and the previous/next chapter
// arrows.
//
// mdBook writes these as native `title` attributes, which the browser renders
// in its own unstyled popup. The title is moved onto a data attribute so the
// native popup never appears, and a pseudo-element draws the tooltip instead.

(() => {
  const SELECTORS = [
      ".left-buttons > *",
      ".right-buttons > *",
      ".nav-chapters",
      ".mobile-nav-chapters",
  ];

  const HOST_ATTR = "data-chrome-tooltip";

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
  };

  if (document.readyState === "loading") {
      document.addEventListener("DOMContentLoaded", adopt);
  } else {
      adopt();
  }
})();
