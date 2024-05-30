import { Show, Suspense, lazy } from "solid-js";
import { Router } from "@solidjs/router";
import { Route } from "node_modules/@solidjs/router/dist/routers"; // Workaround: imported as 'type' normally
import { Spinner, SpinnerType } from "solid-spinner";
import NotFound from "@routes/not-found";
import TechLogo from "@components/tech-logo";
import "./app.scss";

export default function App() {
  return (
    <Router
      root={(props) => (
        <Suspense fallback={Loading()}>
          <>
            <DebugTools enabled />
            {props.children}
          </>
        </Suspense>
      )}
    >
      {/* Optionally Lazy load your routes to avoid loading untill navigated to */}
      <Route path="/" component={lazy(() => import("@routes/home"))} />
      <Route path="*" component={NotFound} />
    </Router>
  );
}

function Loading() {
  return (
    <div class="flex flex-col h-dvh w-dvw justify-center items-center align-middle relative">
      <h1 class="font-bold text-4xl text-cyan-400 drop-shadow-md">
        Loading App
      </h1>
      <TechLogo name="tauri" />
      <Spinner
        type={SpinnerType.rings}
        color="orange"
        class="absolute max-w-96 max-h-96 h-full w-full -z-10"
      />
    </div>
  );
}

function DebugTools(props: { enabled?: boolean }) {
  return (
    <Show when={props.enabled}>
      <div class="absolute right-1 top-1">Debug Placeholder</div>
    </Show>
  );
}
