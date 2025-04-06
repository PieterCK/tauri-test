import $ from "jquery";
import '../styles/styles.css';
import { initRealmRegistrationForm } from "./realm_registration_form";
import { Window } from "@tauri-apps/api/window"
import { Webview } from "@tauri-apps/api/webview"

window.addEventListener("DOMContentLoaded", () => {

  const appWindow = new Window('uniqueLabel');

  // loading embedded asset:
  const webview = new Webview(appWindow, 'theUniqueLabel', {
    url: 'path/to/page.html',
    x:0,
    y:0,
    height:100,
    width:100
  });
  const $mainContainer = $("main.container")[0];
  initRealmRegistrationForm({
    $root: $mainContainer,
    onChange: () => void {},
  });
});
