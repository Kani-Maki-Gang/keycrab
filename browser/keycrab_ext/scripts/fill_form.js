(function() {
  if (window.hasRun) {
    return;
  }
  async function fillForm(d) {
    const domain = document.documentURI;
    // const r = await fetch(`http://localhost:8000/domain/search?q=${"test"}`);
    // const d = await r.json();
    console.log(d);
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
      await fillForm({
        username: message.username,
        password: message.password }
      );
    } else if (message.command === "reset") {
      clearForm();
    }
  });
  window.hasRun = true;
})();
