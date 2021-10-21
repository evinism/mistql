import clsx from 'clsx';
import React from 'react';
import styles from './HomepageFeatures.module.css';

const FeatureList = [
  {
    title: 'Simple Syntax',
    //Svg: require('../../static/img/undraw_docusaurus_tree.svg').default,
    description: (
      <>
        MistQL uses a simple syntax to chain together complicated expressions in a fluent,
        easy to read manner. Readability is a major goal of MistQL.
      </>
    ),
  },
  {
    title: 'Extremely Lightweight',
    //Svg: require('../../static/img/undraw_docusaurus_mountain.svg').default,
    description: (
      <>
        MistQL has 0 dependencies and is hand-tuned for size. At 5.2kB gzipped, MistQL is able to fit on size-restricted frontends.
      </>
    ),
  },
  {
    title: 'Built for Browsers',
    //Svg: require('../../static/img/undraw_docusaurus_react.svg').default,

    description: (
      <>
        MistQL is is purpose-built to be embedded in both browserside and serverside JavaScript.
      </>
    ),
  },
];

function Feature({ Svg, title, description }) {
  return (
    <div className={clsx('col col--4')}>
      {Svg && <div className="text--center">
        <Svg className={styles.featureSvg} alt={title} />
      </div>}
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <>
      <section className={styles.features}>
        <div className="container">
          <div className="row">
            {FeatureList.map((props, idx) => (
              <Feature key={idx} {...props} />
            ))}
          </div>
        </div>
      </section>
      <hr className={styles.tripdot} />
      <section className={styles.prose}>
        <div className="container">
          <h2 className="text--center">How MistQL stacks up against other solutions</h2>
          <h3 className="text--center">JMESPath</h3>
          <p>
            JMESPath and MistQL are similar in scope, but MistQL provides more out of the box, including
            arithmetic operations and regexes. A goal of MistQL is to not simply be a query language,
            but a language for defining ANY computation on a JSON-like object. One of the major
            driving factors behind MistQL is the fact that JMESPath isn't able to capture many
            of the key operations that MistQL provides.
          </p>
          <p>
            JMESPath, however, has excellent cross-language support, whereas MistQL (for the time being)
            does not. If cross-language support is important, JMESPath might be your best bet, as MistQL
            doesn't yet have implementations in multiple languages.
          </p>
          <h3 className="text--center">JSONLogic</h3>
          <p>
            JSONLogic is much smaller than MistQL, but also much less expressive. If the shared
            logic is extremely simple, JSONLogic might work better for you. If JSONLogic isn't
            expressive enough, or readability of JSONLogic becomes difficult, then MistQL will
            probably work better.
          </p>
          <h3 className="text--center">Emuto</h3>
          <p>
            Emuto and MistQL have similar scopes, however Emuto lacks a number of features that mistql
            provides out of the box, including regexes and a large standard library.
          </p>
          <h3 className="text--center">jq</h3>
          <p>
            Despite jq's ubiquity, `jq` doesn't have a strong browser implementation. `jq` may be
            a better choice if all you need is a command line utility, but if you need in-browser
            computation, MistQL is probably a stronger choice.
          </p>
        </div>
      </section>
    </>
  );
}
