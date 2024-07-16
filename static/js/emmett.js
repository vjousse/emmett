function fallbackCopyTextToClipboard(text) {
  var textArea = document.createElement("textarea");
  textArea.value = text;

  // Avoid scrolling to bottom
  textArea.style.top = "0";
  textArea.style.left = "0";
  textArea.style.position = "fixed";

  document.body.appendChild(textArea);
  textArea.focus();
  textArea.select();

  try {
    var successful = document.execCommand("copy");
    var msg = successful ? "successful" : "unsuccessful";
    console.log("Fallback: Copying text command was " + msg);
  } catch (err) {
    console.error("Fallback: Oops, unable to copy", err);
  }

  document.body.removeChild(textArea);
}
function copyTextToClipboard(text, clickedElement) {
  if (!navigator.clipboard) {
    fallbackCopyTextToClipboard(text);
    return;
  }
  navigator.clipboard.writeText(text).then(
    function () {
      console.log("Async: Copying to clipboard was successful!");
      clickedElement.innerHTML =
        '<svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 498.138 498.138" xml:space="preserve"> <g> <path d="M493.65,109.76c-9.859-18.405-32.775-25.333-51.179-15.472 c-22.059,11.816-42.897,23.982-62.82,36.717l0.003-51.276c0-11.313-9.146-20.494-20.493-20.494H20.457 C9.164,59.235,0,68.417,0,79.729v338.7c0,11.291,9.163,20.474,20.457,20.474h338.686c11.348,0,20.496-9.183,20.496-20.474 l0.009-195.875c30.092-22.165,62.312-42.213,98.529-61.615C496.582,151.079,503.509,128.166,493.65,109.76z M338.702,397.917 H40.968V100.219h297.734v58.715c-40.715,29.649-78.022,62.759-114.834,101.677c-4.275-5.648-8.601-11.423-13.129-17.47 c-9.354-12.491-19.958-26.648-32.375-42.632c-12.81-16.487-36.561-19.468-53.05-6.659c-16.488,12.811-19.47,36.562-6.659,53.051 c12.007,15.455,21.949,28.728,31.563,41.565c13.841,18.482,26.915,35.938,42.45,54.771c7.075,8.576,17.566,13.604,28.682,13.745 c0.162,0.002,0.321,0.002,0.482,0.002c10.94,0,21.356-4.741,28.541-13.012c29.482-33.939,58.199-62.952,88.329-88.826V397.917z"/> </g> </svg>';
    },
    function (err) {
      console.error("Async: Could not copy text: ", err);
    }
  );
}
document.addEventListener(
  "DOMContentLoaded",
  function () {
    const divs = document.querySelectorAll(".copy-to-clipboard");

    divs.forEach((el) =>
      el.addEventListener("click", (event) => {
        console.log("CLIC");
        const code = event.currentTarget.nextElementSibling;
        console.log(event.currentTarget);
        console.log(code);
        console.log(code.textContent);
        copyTextToClipboard(code.textContent, event.currentTarget);
      })
    );
  },
  false
);
