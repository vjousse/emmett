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
        '<svg fill="#9acc76" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" class="fill-green" viewBox="0 0 78.369 78.369" xml:space="preserve"> <g> <path d="M78.049,19.015L29.458,67.606c-0.428,0.428-1.121,0.428-1.548,0L0.32,40.015c-0.427-0.426-0.427-1.119,0-1.547l6.704-6.704 c0.428-0.427,1.121-0.427,1.548,0l20.113,20.112l41.113-41.113c0.429-0.427,1.12-0.427,1.548,0l6.703,6.704 C78.477,17.894,78.477,18.586,78.049,19.015z"/> </g> </svg>';
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
