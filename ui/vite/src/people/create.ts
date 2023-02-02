export async function createPerson(params: any): Promise<any> {
  try {
    const res = await fetch("http://localhost:3000/api/people", {
      method: "POST",
      body: JSON.stringify({
        ...params,
        episodeIds: [],
      }),
    });
    const json = await res.json();
    return json;
  } catch (err) {
    return {
      message: (err as Error).message,
    };
  }
}
