import $ from "jquery";
import '../styles/styles.css';
import { initRealmRegistrationForm } from "./realm_registration_form";

window.addEventListener("DOMContentLoaded", () => {
  const $mainContainer = $("main.container")[0];
  initRealmRegistrationForm({
    $root: $mainContainer,
    onChange: () => void {},
  });
});
