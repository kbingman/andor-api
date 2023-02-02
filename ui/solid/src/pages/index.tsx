import { type Component } from "solid-js";
import { People } from "../components/people";
import { Episodes } from "../components/episodes";

const Index: Component = () => {
  return (
    <>
      <People />
      <Episodes />
    </>
  );
};

export default Index;
