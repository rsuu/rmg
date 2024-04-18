// 禁用双指放大
document.documentElement.addEventListener(
  "touchstart",
  function (event) {
    if (event.touches.length > 1) {
      event.preventDefault();
    }
  },
  {
    passive: false,
  },
);

// 禁用双击放大
let lastTouchEnd = 0;
document.documentElement.addEventListener(
  "touchend",
  function (event) {
    const now = Date.now();
    if (now - lastTouchEnd <= 300) {
      event.preventDefault();
    }
    lastTouchEnd = now;
  },
  {
    passive: false,
  },
);
