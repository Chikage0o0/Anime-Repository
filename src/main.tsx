import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

//全局禁止右击
document.addEventListener("contextmenu", function (e) {
  e.preventDefault();
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
