"use strict";

function notify(msg) {
  browser.notifications.create('lel', {
    type: "basic",
    iconUrl: browser.runtime.getURL("icons/kc.jpeg"),
    title: "Time for cake!",
    message: msg,
  });
}

function reportExecuteScriptError(error) {
  document.querySelector("#popup-content").classList.add("hidden");
  document.querySelector("#error-content").classList.remove("hidden");
}

function listenForClicks() {
  function fill(tabs) {
    browser.tabs.sendMessage(tabs[0].id, {
      command: 'fill'
    });
  }

  function reset(tabs) {
    browser.tabs.sendMessage(tabs[0].id, {
      command: 'reset'
    });
  }

  document.addEventListener("click", (e) => {
    if (e.target.tagName !== "BUTTON" || !e.target.closest("#popup-content"))  {
      return;
    }

    if (e.target.type === "reset") {
      browser.tabs.query({active: true, currentWindow: true})
        .then(reset)
        .catch(reportError);
    } else {
      browser.tabs.query({active: true, currentWindow: true})
        .then(fill)
        .catch(reportError);
    }
  });
}

browser.tabs.executeScript({file: "/content_scripts/fillForm.js"})
.then(listenForClicks)
.catch(reportExecuteScriptError);
