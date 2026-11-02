document.addEventListener("DOMContentLoaded", () => {
  const dismissFlash = (flash) => {
    flash.classList.remove("show");
    setTimeout(() => flash.remove(), 300);
  };

  document.querySelectorAll(".flash").forEach((flash) => {
    // إغلاق تلقائي بعد 5 ثوانٍ
    const timer = setTimeout(() => dismissFlash(flash), 5000);

    // إغلاق يدوي عند الضغط على الزر
    const closeBtn = flash.querySelector(".flash__close");
    if (closeBtn) {
      closeBtn.addEventListener("click", () => {
        clearTimeout(timer); // إيقاف العداد التلقائي
        dismissFlash(flash);
      });
    }
  });
});

/* ==========================================================================
   3. Global Outside Click Handler (Dropdowns & Active Overlays)
   ========================================================================== */

window.addEventListener("click", (e) => {
  // Close any search results dropdown when clicking outside the container
  const searchContainer = document.querySelector(".search-container");
  const searchResults = document.getElementById("search-results");

  if (searchContainer && searchResults && !searchContainer.contains(e.target)) {
    searchResults.innerHTML = "";
  }
});

document.addEventListener("click", function (event) {
  const searchContainer = document.querySelector(".search-container");
  const dropdown = document.getElementById("search-results-dropdown");

  // إذا كانت القائمة موجودة والنقر تم خارج .search-container
  if (dropdown && searchContainer && !searchContainer.contains(event.target)) {
    dropdown.innerHTML = "";
  }
});

// إغلاق القائمة عند الضغط على زر Escape
document.addEventListener("keydown", function (event) {
  if (event.key === "Escape") {
    const dropdown = document.getElementById("search-results-dropdown");
    if (dropdown) dropdown.innerHTML = "";
  }
});

document.querySelectorAll(".delete-form").forEach((form) => {
  form.addEventListener("submit", function (e) {
    const confirmed = confirm("تنبيه: هل أنت متأكد من رغبتك في حذف هذه الفئة؟");
    if (!confirmed) {
      e.preventDefault(); // Stop form submission
    }
  });
});

/* ==========================================================================
   4. For attributes in product variants to add or delete
   ========================================================================== */

function addAttrRow() {
  const container = document.getElementById("attributes-container");
  const row = document.createElement("div");
  row.className = "attr-row";
  row.style.cssText =
    "display: flex; gap: 10px; align-items: center; margin-bottom: 8px;";
  row.innerHTML = `
        <input type="text" name="attr_keys[]" placeholder="الخاصية (Key e.g. size)" style="flex: 1;">
        <input type="text" name="attr_values[]" placeholder="القيمة (Value e.g. A4)" style="flex: 1;">
        <button type="button" onclick="this.parentElement.remove()" style="padding: 6px 12px; background: #dc2626; color: white; border: none; border-radius: 4px; cursor: pointer;">✕</button>
    `;
  container.appendChild(row);
}