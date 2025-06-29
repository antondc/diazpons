import { onDomReady } from '../../utils/onDomReady.ts';

const ANIMATION_DURATION = 10000;

onDomReady(() => {
  console.log('Home JavaScript loaded.');

  const slides = Array.from(document.getElementsByClassName('Home-slide'));

  if (slides.length === 0) return;

  function animateSlides(index: number) {
    // Get current slide
    const slide = slides[index];
    // Get previous slide or last one
    const previousSlide = slides[index - 1] || slides[slides.length - 1];

    slide.classList.add('Home-slide--shown');
    previousSlide.classList.remove('Home-slide--shown');

    const nextIndex = (index + 1) % slides.length;

    setTimeout(() => animateSlides(nextIndex), ANIMATION_DURATION);
  }

  animateSlides(0);
});
