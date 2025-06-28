export const onDomReady = (callBack: () => void) => {
  document.readyState === 'interactive' || document.readyState === 'complete'
    ? callBack()
    : document.addEventListener('DOMContentLoaded', callBack);
};
