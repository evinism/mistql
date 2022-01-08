import clsx from "clsx";
import React from "react";
import styles from "./HomepageFeatures.module.css";

const FeatureList = [
  {
    title: "Simple Syntax",
    //Svg: require('../../static/img/undraw_docusaurus_tree.svg').default,
    description: (
      <>
        MistQL uses a simple syntax to chain together complicated expressions in
        a fluent, easy to read manner. Readability is a major goal of MistQL.
      </>
    ),
  },
  {
    title: "Built for Browsers",
    //Svg: require('../../static/img/undraw_docusaurus_mountain.svg').default,
    description: (
      <>
        MistQL's NPM implementation has 0 dependencies and is hand-tuned for size. 
        At 5.5kB gzipped, MistQL is specifically built to be embedded in size-restricted
        frontends.
      </>
    ),
  },
  {
    title: "Frontend ⇆ Backend",
    //Svg: require('../../static/img/undraw_docusaurus_react.svg').default,

    description: (
      <>
        MistQL has both a JavaScript and Python implementation,
        and can be used in both clientside and serverside. Sharing
        functions between your frontend and your backend has 
        never been easier!
      </>
    ),
  },
];

function Feature({ Svg, title, description }) {
  return (
    <div className={clsx("col col--4")}>
      {Svg && (
        <div className="text--center">
          <Svg className={styles.featureSvg} alt={title} />
        </div>
      )}
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
      <section className={styles.tada}>
        <div className="text--center padding-horiz--md">
          <h3> 🎉🎉🎉 New: MistQL 0.4.4 now supports Python! 🎉🎉🎉</h3>
          <p>You can visit the <a href="docs/intro">getting started</a> page to begin!</p>
        </div>
      </section>
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
          <h2 className="text--center">Select Examples</h2>
          <p className="text--center">
            More examples can be found by navigating to the "Try It Now" link.
          </p>
          <h3 className="text--center">Get count of a specific event</h3>
          <pre>events | filter type == "submit" | count</pre>
          <h3 className="text--center">Get count of all event types</h3>
          <pre>events | groupby type | mapvalues count</pre>
          <h3 className="text--center">Get the worst possible chess line</h3>
          <pre>(lines | sortby overallScore)[-1]</pre>
          <h3 className="text--center">
            Get emails of all users that use the Chat feature
          </h3>
          <pre>
            events | filter type == "send_message" | groupby email | keys
          </pre>
          <h3 className="text--center">
            Get usernames of all users who purchased before signing up
          </h3>
          <pre>
            events | sort timestamp | groupby email | mapvalues (sequence type
            == "purchase" type == "signup") | filtervalues (count @ {">"} 0) |
            keys
          </pre>
        </div>
      </section>
      <hr className={styles.tripdot} />
      <section className={styles.prose}>
        <div className="container">
          <h2 className="text--center">Motivation</h2>
          <p>
            MistQL originated as a domain specific language for machine learning
            feature extraction on the frontend. Despite the widespread
            proliferation of learned features in neural networks, the need for
            handcrafted features still provides immense value in situations
            where classical machine learning models are more appropriate. MistQL
            was purpose-built to fill the niche of a domain specific language
            for feature extraction on the frontend.
          </p>
          <p>
            Despite this initial motivation, MistQL is far more general than
            simply clientside feature extraction. MistQL can be used in a wide
            range of applications, including, but not limited to:
            <ul>
              <li>User-submitted or untrusted logic</li>
              <li>
                Shared backend / frontend logic in scenarios where sharing code
                is infeasable
              </li>
              <li>As a serializable storage format for pure functions</li>
            </ul>
          </p>
        </div>
      </section>
      <hr className={styles.tripdot} />
      <section className={styles.prose}>
        <div className="container">
          <h2 className="text--center">
            How MistQL stacks up against other solutions
          </h2>
          <h3 className="text--center">JMESPath</h3>
          <p>
            JMESPath and MistQL are similar in scope, but MistQL provides more
            out of the box, including arithmetic operations and regexes. A goal
            of MistQL is to not simply be a query language, but a language for
            defining a broad set of computations on a JSON-like object. One of 
            the major driving factors behind MistQL is the fact that JMESPath 
            isn't able to capture many of the key operations that MistQL provides.
          </p>
          <p>
            JMESPath, however, has excellent cross-language support, whereas
            MistQL (for the time being) does not. If cross-language support is
            important, JMESPath might be your best bet, as MistQL only supports
            Python and JavaScript.
          </p>
          <h3 className="text--center">JSONLogic</h3>
          <p>
            JSONLogic is much smaller than MistQL, but also much less
            expressive. If the shared logic is extremely simple, JSONLogic might
            work better for you. If JSONLogic isn't expressive enough, or
            readability of JSONLogic becomes difficult, then MistQL will
            probably work better.
          </p>
          <h3 className="text--center">jq</h3>
          <p>
            jq is primarily used as a command-line tool rather than as an
            embeddable query language. Despite its ubiquity, jq doesn't have a
            robust, portable browser implementation, nor does it seem to be a
            major goal of the project. However, if your primary use case is as a
            CLI tool, jq is almost certainly your best bet, as it's a very
            well-known tool with a large amount of community support.
          </p>
          <p>
            The decision criteria for whether to use jq or MistQL is very
            similar to the decision criteria for whether to use JMESPath or jq.
          </p>
        </div>
      </section>
    </>
  );
}
