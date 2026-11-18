// Sorting for card grids that opt in through a `.card-sort` control.
//
// Each card states what it can be sorted by in its own markup, so this script
// never carries a second copy of the data: the name is the `.title-name` text,
// the group is the `.card-district` text when a card has one and the
// `.category-icon` tooltip otherwise, and the layers are the order of the
// `.layer-icon` entries, whose tooltips are the layer display names.
//
// Only the creature grid has a fixed order to rank groups by, so the group
// comparators sort alphabetically. Ranking by a list would give every trader the
// same fallback rank - its district is not in the creature category list - and the
// sort would silently do nothing.

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

    const rank = (list, value) => {
        const index = list.indexOf(value);
        return index === -1 ? list.length : index;
    };

    const text = (element, selector) => {
        const found = element.querySelector(selector);
        return found ? found.textContent.trim() : "";
    };

    const nameOf = (card) => text(card, ".title-name");

    const groupOf = (card) => {
        const district = card.querySelector(".card-district");
        if (district) return district.textContent.trim();
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
            groupOf(a).localeCompare(groupOf(b)) ||
            nameOf(a).localeCompare(nameOf(b)),
        "category-desc": (a, b) =>
            groupOf(b).localeCompare(groupOf(a)) ||
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
        document.querySelectorAll(".card-sort").forEach((control) => {
            const grid = document.getElementById(control.dataset.sortFor);
            const select = control.querySelector("select");
            if (!grid || !select) return;

            select.addEventListener("change", () =>
                sortGrid(grid, select.value),
            );
            sortGrid(grid, select.value);
        });
    };

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", init);
    } else {
        init();
    }
})();
