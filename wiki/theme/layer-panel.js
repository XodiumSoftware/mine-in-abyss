// Expands a layer hotspot's tooltip into a detail card when it is clicked.
//
// Each hotspot holds its own details, so there is no second copy of the text:
// the tip always shows the name, and the facts and prose inside it are revealed
// only while the hotspot carries the `expanded` class. One is open at a time.

(() => {
    const init = () => {
        const map = document.querySelector(".layer-map");
        if (!map) return;

        const hotspots = Array.from(map.querySelectorAll(".hotspot"));

        const collapse = (hotspot) => {
            hotspot.classList.remove("expanded");
            hotspot.setAttribute("aria-expanded", "false");
        };

        const expand = (hotspot) => {
            hotspots.forEach(collapse);
            hotspot.classList.add("expanded");
            hotspot.setAttribute("aria-expanded", "true");
        };

        hotspots.forEach((hotspot) => {
            hotspot.setAttribute("aria-expanded", "false");

            hotspot.addEventListener("click", () => {
                if (hotspot.classList.contains("expanded")) {
                    collapse(hotspot);
                } else {
                    expand(hotspot);
                }
            });
        });

        document.addEventListener("keydown", (event) => {
            if (event.key === "Escape") hotspots.forEach(collapse);
        });
    };

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", init);
    } else {
        init();
    }
})();
