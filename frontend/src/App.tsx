import { AppShell } from "./components/AppShell";
import { GrafiTalkSplash } from "./components/grafi-splash/GrafiTalkSplash";
import { useStartupSplash } from "./hooks/useStartupSplash";

function App() {
  const { appReady, splashMounted, markAppReady, handleSplashHidden } =
    useStartupSplash();

  return (
    <>
      <AppShell onStartupReady={markAppReady} />
      {splashMounted ? (
        <GrafiTalkSplash visible={!appReady} onHidden={handleSplashHidden} />
      ) : null}
    </>
  );
}

export default App;
