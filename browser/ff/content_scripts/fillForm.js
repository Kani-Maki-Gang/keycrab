(function() {
  if (window.hasRun) {
    return;
  }
  async function fillForm() {
    const r = await fetch("http://localhost:8000/password");
    const domain = document.documentURI;
    // const r = await fetch(`http://localhost:8000/domain?domain=${"test"}`);
    const d = await r.json();
    const user =  document.getElementById("username");
    const pwd =  document.getElementById("password");
    user.value = d.username;
    pwd.value = d.password;
  }

  function clearForm() {
    const user =  document.getElementById("username");
    const pwd =  document.getElementById("password");
    user.value = "";
    pwd.value = "";
  }

  browser.runtime.onMessage.addListener(async (message) => {
    if (message.command === "fill") {
      await fillForm();
    } else if (message.command === "reset") {
      clearForm();
    }
  });
  window.hasRun = true;
})();
