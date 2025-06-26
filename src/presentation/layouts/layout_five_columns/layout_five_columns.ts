console.log('LayoutFiveColumnsLoaded JavaScript loaded.');

const element = document.getElementById('body');
if (element) {
  element.classList.add('isLoaded');
}

const images = document.querySelectorAll('img');

images.forEach((img) => {
  if (img.complete) {
    // Already loaded
    img.classList.add('Image-loaded');
  } else {
    // Wait for the load event
    img.addEventListener('load', () => {
      img.classList.add('Image-loaded');
    });

    img.addEventListener('error', () => {
      console.warn('Image failed to load:', img.src);
    });
  }
});
