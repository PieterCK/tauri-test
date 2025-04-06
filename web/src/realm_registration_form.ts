import { generateNodeFromHtml, html } from "./html";
import { getServerSettings } from "./server_settings";

type RealmRegistrationFormProperties = {
  $root: Element;
  onChange: () => void;
};

export function initRealmRegistrationForm({
  $root,
  onChange,
}: RealmRegistrationFormProperties): void {
  const $realmRegistrationForm = generateNodeFromHtml(
    html`
      <div class="">
        <h1>Welcome to Zulip</h1>

        <div class="row">
          <a href="https://zulip.com/" target="_blank">
            <img
              src="./images/zlp_logo.png"
              class="logo typescript"
              alt="typescript logo"
            />
          </a>
        </div>
        <p>
          Enter your Zulip server URL:
          <a href="https://zulip.com/">(What's this?)</a>
        </p>

        <form class="row" id="realm-registration-form">
          <input id="url-input" placeholder="your-org.zulipchat.com" />
          <button type="submit">Enter</button>
        </form>
        <p id="status-msg"></p>
      </div>
    `
  );
  $root.textContent = "";
  $root.append($realmRegistrationForm);

  const $registrationForm = $realmRegistrationForm.querySelector(
    "#realm-registration-form"
  )!;
  const $urlInput: HTMLInputElement =
    $realmRegistrationForm.querySelector("#url-input")!;

  const $statusMsg = $realmRegistrationForm.querySelector("#status-msg")!;
  async function registrationFormHandler() {
    $statusMsg.textContent = "Connecting...";

    const domain = $urlInput.value.trim();
    getServerSettings(domain)
      .then(() => {
        $statusMsg.textContent = `Successfully connected ${domain}`;
        onChange();
      })
      .catch((e) => {
        $statusMsg.textContent = `Fail to connect to ${domain}`;
        console.error("Error connecting to server:", e);
      });
  }

  $registrationForm.addEventListener("submit", (e) => {
    e.preventDefault();
    registrationFormHandler();
  });
}
