// Sorting for card grids that opt in through a `.creature-sort` control.
//
// Each card states what it can be sorted by in its own markup, so this script
// never carries a second copy of the data: the name is the `.title-name` text,
// the category is the `.category-icon` tooltip, and the layers are the order of
// the `.layer-icon` entries, whose tooltips are the layer display names.

(() => {
  const LAYER_ORDER = [
    "The City on the Edge",
    "Depths of the Abyss",
    "The Forest of Temptation",
    "The Great Fault",
    "The Goblets of Giants",
    "The Sea of Corpses",
    "The Capital of the Unreturned",
    "The Final Maelstrom",
  ];

  const CATEGORY_ORDER = ["Passive", "Docile", "Aerial", "Aquatic", "Hostile"];

  const rank = (list, value) => {
    const index = list.indexOf(value);
    return index === -1 ? list.length : index;
  };

  const text = (element, selector) => {
    const found = element.querySelector(selector);
    return found ? found.textContent.trim() : "";
  };

  const nameOf = (card) => text(card, ".title-name");

  const categoryOf = (card) => {
    const icon = card.querySelector(".category-icon");
    return icon ? icon.dataset.tooltip || "" : "";
  };

  const layersOf = (card) =>
    Array.from(card.querySelectorAll(".layer-icon")).map(
      (icon) => icon.dataset.tooltip || "",
    );

  // A creature's layer is its shallowest one, so a kind living in two layers sorts
  // by the upper of them.
  const firstLayerOf = (card) => {
    const layers = layersOf(card).map((layer) => rank(LAYER_ORDER, layer));
    return layers.length === 0 ? LAYER_ORDER.length : Math.min(...layers);
  };

  const COMPARATORS = {
    "name-asc": (a, b) => nameOf(a).localeCompare(nameOf(b)),
    "name-desc": (a, b) => nameOf(b).localeCompare(nameOf(a)),
    "layer-asc": (a, b) =>
      firstLayerOf(a) - firstLayerOf(b) ||
      nameOf(a).localeCompare(nameOf(b)),
    "layer-desc": (a, b) =>
      firstLayerOf(b) - firstLayerOf(a) ||
      nameOf(a).localeCompare(nameOf(b)),
    "category-asc": (a, b) =>
      rank(CATEGORY_ORDER, categoryOf(a)) -
        rank(CATEGORY_ORDER, categoryOf(b)) ||
      nameOf(a).localeCompare(nameOf(b)),
    "category-desc": (a, b) =>
      rank(CATEGORY_ORDER, categoryOf(b)) -
        rank(CATEGORY_ORDER, categoryOf(a)) ||
      nameOf(a).localeCompare(nameOf(b)),
  };

  const sortGrid = (grid, mode) => {
    const compare = COMPARATORS[mode];
    if (!compare) return;

    const cards = Array.from(grid.children).filter((node) =>
      node.classList.contains("recipe-card"),
    );

    cards.sort(compare).forEach((card) => grid.appendChild(card));
  };

  const init = () => {
    document.querySelectorAll(".creature-sort").forEach((control) => {
      const grid = document.getElementById(control.dataset.sortFor);
      const select = control.querySelector("select");
      if (!grid || !select) return;

      select.addEventListener("change", () => sortGrid(grid, select.value));
      sortGrid(grid, select.value);
    });
  };

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();
