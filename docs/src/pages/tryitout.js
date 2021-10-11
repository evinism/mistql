import BrowserOnly from '@docusaurus/BrowserOnly';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import React from 'react';
import TryItOut from "../components/TryItOut";


export default function TryItOutPage() {
  useDocusaurusContext();
  return (
    <Layout
      title={`Try It Out`}
      description="MistQL: A miniature language for querying JSON-like structures">
      <main>
        <BrowserOnly>
          {() => <TryItOut />}
        </BrowserOnly>
      </main>
    </Layout>
  );
}
