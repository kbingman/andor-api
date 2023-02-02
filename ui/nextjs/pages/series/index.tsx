import Head from "next/head";
import { GetStaticProps } from "next";

export const getStaticProps: GetStaticProps = (context) => {
  console.log(context);
  return {
    props: {}, // will be passed to the page component as props
  };
};

export default function Home() {
  return (
    <div className="container">
      <Head>
        <title>Series</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main>
        <h1 className="h1">Series Api</h1>
      </main>

      <style jsx>{`
        .container {
          margin: 16px;
        }
      `}</style>
    </div>
  );
}
