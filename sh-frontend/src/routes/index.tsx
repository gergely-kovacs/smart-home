import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/")({
  component: App,
});

function App() {
  return (
    <main className="grow flex flex-col items-center justify-center bg-[#282c34] text-white text-[calc(10px+2vmin)]"></main>
  );
}
