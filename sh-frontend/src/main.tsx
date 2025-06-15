import { ColorModeProvider } from "@/providers/ColorMode.tsx";
import { ChakraProvider, createSystem, defaultConfig } from "@chakra-ui/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import reportWebVitals from "./reportWebVitals.ts";
import { routeTree } from "./routeTree.gen";
import "./styles.css";

const queryClient = new QueryClient();

const router = createRouter({
  routeTree,
  context: {},
  defaultPreload: "intent",
  scrollRestoration: true,
  defaultStructuralSharing: true,
  defaultPreloadStaleTime: 0,
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

export const chakraTheme = createSystem(defaultConfig, {
  theme: {
    tokens: {
      fonts: {
        heading: { value: `'Roboto', sans-serif` },
        body: { value: `'Roboto', sans-serif` },
      },
    },
  },
});

const rootElement = document.getElementById("app");

if (rootElement && !rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement);
  root.render(
    <StrictMode>
      <ChakraProvider value={chakraTheme}>
        <ColorModeProvider>
          <QueryClientProvider client={queryClient}>
            <RouterProvider router={router} />
          </QueryClientProvider>
        </ColorModeProvider>
      </ChakraProvider>
    </StrictMode>,
  );
}

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
