export function applyMasonry(container: HTMLElement) {
  const rowHeight = 1;
  const cardMargin = 32; // 2rem margin-bottom

  const resize = () => {
    container.querySelectorAll<HTMLElement>('.chapter-card').forEach(card => {
      const height = Math.max(
        card.getBoundingClientRect().height,
        card.scrollHeight,
        card.offsetHeight
      );
      const rows = Math.ceil((height + cardMargin) / rowHeight);
      card.style.gridRowEnd = `span ${rows}`;
    });
  };

  resize();
  window.addEventListener('resize', resize);
  return () => window.removeEventListener('resize', resize);
}
