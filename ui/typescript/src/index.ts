import { HandleRequest, HttpRequest, HttpResponse } from "@fermyon/spin-sdk";
import { styles } from "./styles";
import { html } from "./template";

const encoder = new TextEncoder();

export const handleRequest: HandleRequest = async (
  _request: HttpRequest
): Promise<HttpResponse> => {
  const { buffer } = encoder.encode(
    html({
      styles,
      title: "Andor API",
    })
  );

  return {
    status: 200,
    headers: { "Content-Type": "text/html" },
    body: buffer,
  };
};
