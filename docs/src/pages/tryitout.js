import React from 'react';
import Layout from '@theme/Layout';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import TryItOut from "../components/TryItOut";

export default function TryItOutPage() {
  useDocusaurusContext();
  return (
    <Layout
      title={`Try It Out`}
      description="MistQL: A miniature language for querying JSON-like structures">
      <main>
        <TryItOut />
      </main>
    </Layout>
  );
}
