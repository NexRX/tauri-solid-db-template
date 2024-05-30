import solidLogo from "@assets/logo.svg";
import "./tech-logo.scss"

interface TechLogoProps {
  name: "vite" | "tauri" | "solidjs";
}

export default function TechLogo(props: TechLogoProps) {
  return (
    <a href={techLink[props.name]} target="_blank">
      <img
        src={imgSrc[props.name]}
        class={`logo ${props.name}`}
        alt={`${props.name} logo`}
      />
    </a>
  );
}

const imgSrc = {
  vite: "/vite.svg",
  tauri: "/tauri.svg",
  solidjs: solidLogo,
} as const;

const techLink = {
  vite: "https://vitejs.dev",
  tauri: "https://tauri.app",
  solidjs: "https://solidjs.com",
} as const;
