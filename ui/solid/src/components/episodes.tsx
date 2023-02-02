import { type Component, createResource, For } from "solid-js";

const fetchEpisodes = (api: string) => async () => {
  return (await fetch(`${api}/api/episodes`)).json();
};

export const Episodes: Component = () => {
  const api = "http://localhost:3000";
  const [episodes] = createResource(fetchEpisodes(api));

  return (
    <div class="p-2">
      <h2 class="text-md font-bold">Episodes</h2>
      <div class="text-sm">{episodes.loading && "Loading..."}</div>
      <For each={episodes()}>{({ title }) => <div>{title}</div>}</For>
    </div>
  );
};
