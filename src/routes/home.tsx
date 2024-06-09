import { For, createSignal, onMount } from "solid-js";
import MainLayout from "@layouts/main-layout";
import TechLogo from "@components/tech-logo";
import { commands, Greeting } from "../bindings";

export default function Home() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [greetings, setGreetings] = createSignal<Greeting[]>([]);

  onMount(() => {
    commands
      .getAllGrettings()
      .then((result) =>
        result.status === "ok" ? setGreetings(result.data) : []
      );
  });

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
          commands.createGreeting(greetMsg());
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setGreetMsg(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <button
        onClick={async () => {
          const greetings = await commands.getAllGrettings();
          if (greetings.status === "ok") {
            console.log(greetings);
            setGreetings(greetings.data);
          } else {
            console.error(greetings);
          }
        }}
      >
        Refresh
      </button>
      <For each={greetings()}>{(greeting) => <p>{greeting.message}</p>}</For>
      <a type="submit" href="/bad-route">
        Get Lost
      </a>
    </MainLayout>
  );
}
