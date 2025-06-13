document.addEventListener('DOMContentLoaded', function () {
  console.log('DoubleArrowUp JavaScript loaded.');

  const element = document.getElementById('double-arrow-up');

  element?.addEventListener('click', function () {
    window.scrollTo({ top: 0, behavior: 'smooth' });
  });
});
