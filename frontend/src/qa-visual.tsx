/* eslint-disable react-refresh/only-export-components -- QA-only visual entry */
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { GrafiTalkSplash } from "./components/grafi-splash/GrafiTalkSplash";
import "./index.css";
import "./styles/ambient.css";

function QaVisual() {
  return <GrafiTalkSplash visible onHidden={() => undefined} />;
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <QaVisual />
  </StrictMode>
);
