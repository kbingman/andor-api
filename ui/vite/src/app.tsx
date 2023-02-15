import { useLoaderData } from "react-router-dom";
import { createPerson } from "./people/create";

type Results = {
  people: any[];
  episodes: any[];
};

const getFormJSON = (formData: FormData) =>
  Array.from(formData.entries()).reduce(
    (memo, [key, value]) => ({
      ...memo,
      [key]: value,
    }),
    {}
  );

// const getFormData = (form: HTMLFormElement) => new FormData(form);

export const App = () => {
  const { people, episodes } = useLoaderData() as Results;

  const peopleStore = people.reduce(
    (acc, person) => {
      acc.entries[person.id] = person;
      acc.ids.push(person.id);
      return acc;
    },
    {
      ids: [],
      entries: {},
    }
  );
  console.log(peopleStore);

  return (
    <div className="app">
      <h1>Andor API</h1>

      <div>
        {episodes.map(({ episode, id, description, title, peopleIds }) => (
          <div key={`episode-${id}`}>
            <h4>
              {episode}. {title}
            </h4>
            <p>{description}</p>
            <div>
              {peopleIds.map((id: string) => (
                <span key={id}>{peopleStore.entries[id]?.name}, </span>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default App;
