import { createSignal } from "solid-js";
import MainLayout from "@layouts/main-layout";
import TechLogo from "@components/tech-logo";
import { invoke } from "@tauri-apps/api/core";

export default function Home() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name: name() }));
  }

  return (
    <MainLayout>
      <h1>Welcome to Tauri!</h1>

      <div class="flex flex-row">
        <TechLogo name="vite" />
        <TechLogo name="tauri" />
        <TechLogo name="solidjs" />
      </div>

      <p>Click on the Tauri, Vite, and Solid logos to learn more.</p>

      <form
        class="flex flex-row gap-2"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg()}</p>
      <a type="submit" href="/bad-route">Get Lost</a>
    </MainLayout>
  );
}
