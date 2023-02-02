import { type Component, createResource, For } from "solid-js";

const fetchPeople = (api: string) => async () => {
  return (await fetch(`${api}/api/people`)).json();
};

export const People: Component = () => {
  const api = "http://localhost:3000";
  const [people] = createResource(fetchPeople(api));

  return (
    <div class="p-2">
      <h2 class="text-md font-bold">People</h2>
      <div class="text-sm">{people.loading && "Loading..."}</div>
      <For each={people()?.results}>{(item) => <div>{item.name}</div>}</For>
    </div>
  );
};
