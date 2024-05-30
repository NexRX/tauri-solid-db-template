import type { JSXElement } from "solid-js";

interface MainLayoutProps {
  children?: JSXElement;
}

export default function MainLayout(props: MainLayoutProps) {
  return (
    <main class="prose dark:prose-invert lg:prose-xl flex flex-col justify-center items-center text-center w-full max-w-full h-full max-h-full p-2">
      {props.children}
    </main>
  );
}
