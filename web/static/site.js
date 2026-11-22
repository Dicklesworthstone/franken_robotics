const query = document.querySelector("#query");

document.addEventListener("keydown", (event) => {
  if (event.key !== "/" || event.altKey || event.metaKey || event.ctrlKey || event.shiftKey) return;
  if (event.target.closest("input, textarea, select, [contenteditable]")) return;
  event.preventDefault();
  if (query) query.focus();
  else location.assign("/search/");
});

const stances = [
  {
    force: 165,
    title: "Beyond the foot limit",
    detail:
      "At the positive load corner, two diagonal foot forces would have to differ by 165 N. Forces between 0 and 160 N cannot meet that requirement.",
  },
  {
    force: 160,
    title: "Exactly at the limit",
    detail:
      "One feasible allocation is 160, 130, 110 and 0 N at the positive load corner. Symmetry covers the other corners.",
  },
  {
    force: 150,
    title: "Within the foot limit",
    detail:
      "The same load corner can be balanced with 155, 135, 105 and 5 N. Each foot stays inside its limit.",
  },
];

for (const button of document.querySelectorAll("[data-stance]")) {
  button.addEventListener("click", () => {
    const stance = stances[Number(button.dataset.stance)];
    for (const sibling of document.querySelectorAll("[data-stance]")) {
      sibling.setAttribute("aria-pressed", String(sibling === button));
    }
    document.querySelector("#stance-title").textContent = stance.title;
    document.querySelector("#stance-detail").textContent = stance.detail;
    document.querySelector("#force").textContent = stance.force + " N";
    document.querySelector(".gauge .fill").style.width = (100 * stance.force) / 200 + "%";
  });
}

function excerptWithHighlights(excerpt) {
  // Pagefind escapes source text and wraps matching words in mark elements.
  const fragment = document.createDocumentFragment();
  const decoder = document.createElement("textarea");
  let parent = fragment;
  for (const part of excerpt.split(/(<\/?mark>)/i)) {
    if (part === "<mark>") {
      parent = document.createElement("mark");
      fragment.append(parent);
    } else if (part === "</mark>") {
      parent = fragment;
    } else {
      decoder.innerHTML = part.replaceAll("<", "&lt;").replaceAll(">", "&gt;");
      parent.append(document.createTextNode(decoder.value));
    }
  }
  return fragment;
}

if (query) {
  const form = query.form;
  const results = document.querySelector("#results");
  const status = document.querySelector("#status");
  let index;
  let generation = 0;
  let timer;

  async function search(term, revision) {
    if (!term) return;
    try {
      index ??= import("/pagefind/pagefind.js");
      const engine = await index;
      const response = await engine.search(term);
      let offset = 0;
      async function next() {
        const documents = await Promise.all(
          response.results.slice(offset, offset + 12).map((hit) => hit.data()),
        );
        if (revision !== generation) return;
        for (const doc of documents) {
          const best = doc.sub_results?.[0];
          const link = document.createElement("a");
          link.className = "result";
          link.href = best?.url || doc.url;
          const name = document.createElement("strong");
          name.textContent = doc.meta.title;
          const passage = document.createElement("p");
          passage.append(excerptWithHighlights(best?.excerpt || doc.excerpt));
          link.append(name);
          if (best?.title && best.title !== doc.meta.title) {
            const section = document.createElement("small");
            section.textContent = best.title;
            link.append(section);
          }
          link.append(passage);
          results.append(link);
        }
        offset += documents.length;
        const count = response.results.length;
        status.textContent = count
          ? offset + " of " + count + " matching chapters"
          : "No matching chapters. Try a shorter phrase or browse the plan library.";
        if (offset < count) {
          const more = document.createElement("button");
          more.className = "secondary";
          more.textContent = "Show more chapters";
          more.addEventListener(
            "click",
            () => {
              more.remove();
              next().catch(() => showError(revision));
            },
            { once: true },
          );
          results.append(more);
        }
      }
      if (revision === generation) await next();
    } catch {
      index = undefined;
      showError(revision);
    }
  }
  function showError(revision) {
    if (revision === generation)
      status.textContent = "Search could not load. Try again, or open the plan library.";
  }
  function update(immediate = false) {
    const term = query.value.trim();
    const revision = ++generation;
    clearTimeout(timer);
    results.replaceChildren();
    status.textContent = term ? "Searching the plan…" : "";
    if (immediate) search(term, revision);
    else timer = setTimeout(() => search(term, revision), 180);
  }
  function setLocation() {
    const url = new URL(location.href);
    if (query.value.trim()) url.searchParams.set("q", query.value.trim());
    else url.searchParams.delete("q");
    history.replaceState(null, "", url);
  }
  query.addEventListener("input", () => {
    setLocation();
    update();
  });
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    setLocation();
    update(true);
  });
  function restore() {
    query.value = new URL(location.href).searchParams.get("q") || "";
    update(true);
  }
  window.addEventListener("popstate", restore);
  restore();
}
