import {
  createSignal,
  onCleanup,
} from "https://cdn.skypack.dev/solid-js";
import { render } from "https://cdn.skypack.dev/solid-js/web";
import html from "https://cdn.skypack.dev/solid-js/html";

const App = () => {
  const [count, setCount] = createSignal(0), timer = setInterval(() => setCount(count() + 1), 1000);
  onCleanup(() => clearInterval(timer));
  return html`
  <h1 class="underline md:bg-green-200 bg-red-200 font-mono text-[40px]"> Hello World </h1>
  <div class="text-5xl"> ${count}</div>
`;
}

render(App, document.body);
