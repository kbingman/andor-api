export const html = ({ styles, title }: any) => `
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>${title}</title>
    <style>
      ${styles}
    </style>
  </head>
  <body>
    <div id="root">
      Hello World
    </div>
  </body>
</html>
`;
