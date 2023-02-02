import Head from "next/head";
import { GetStaticProps } from "next";

export const getStaticProps: GetStaticProps = (_ctx) => {
  return {
    props: {}, // will be passed to the page component as props
  };
};

export default function Home() {
  return (
    <div className="container">
      <Head>
        <title>Andor API</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main>
        <h1 className="h1">Andor Api</h1>
      </main>

      <style jsx>{`
        .container {
          margin: 16px;
        }
      `}</style>
    </div>
  );
}
