import clsx from 'clsx';
import React from 'react';
import styles from './HomepageFeatures.module.css';

const FeatureList = [
  {
    title: 'Simple Syntax',
    Svg: require('../../static/img/undraw_docusaurus_tree.svg').default,
    description: (
      <>
        MistQL uses a simple syntax to chain together complicated expressions in a fluent,
        easy to read manner. Readability is a major goal of MistQL.
      </>
    ),
  },
  {
    title: 'Extremely Lightweight',
    Svg: require('../../static/img/undraw_docusaurus_mountain.svg').default,
    description: (
      <>
        MistQL has 0 dependencies and is hand-tuned for size. At 5.2kB gzipped, MistQL is able to fit on heavily size-restricted frontends.
      </>
    ),
  },
  {
    title: 'Built for Browsers',
    Svg: require('../../static/img/undraw_docusaurus_react.svg').default,

    description: (
      <>
        MistQL is is purpose-built to be embedded in both browserside and serverside JavaScript.
        Serverside implementations include NodeJS and (soon) Python.
      </>
    ),
  },
];

function Feature({ Svg, title, description }) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} alt={title} />
      </div>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
