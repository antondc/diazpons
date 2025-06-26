document.addEventListener('DOMContentLoaded', function () {
  console.log('Header JavaScript loaded.');

  const menu = document.getElementById('header-menu');
  const header = document.getElementById('header');
  menu?.addEventListener('click', () => {
    const headerIsOpen = header?.classList.contains('Header--open');
    if (headerIsOpen) {
      header?.classList.remove('Header--open');
    } else {
      header?.classList.add('Header--open');
    }
  });
});
