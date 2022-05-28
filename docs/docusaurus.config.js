const lightCodeTheme = require("prism-react-renderer/themes/github");
const darkCodeTheme = require("prism-react-renderer/themes/dracula");

// With JSDoc @type annotations, IDEs can provide config autocompletion
/** @type {import('@docusaurus/types').DocusaurusConfig} */
(
  module.exports = {
    title: "MistQL",
    tagline:
      "A query language for JSON-like structures",
    url: "https://mistql.com",
    baseUrl: "/",
    onBrokenLinks: "throw",
    onBrokenMarkdownLinks: "warn",
    favicon: "img/icon128.png",
    organizationName: "evinism", // Usually your GitHub org/user name.
    projectName: "mistql", // Usually your repo name.

    presets: [
      [
        "@docusaurus/preset-classic",
        /** @type {import('@docusaurus/preset-classic').Options} */
        ({
          docs: {
            sidebarPath: require.resolve("./sidebars.js"),
            // Please change this to your repo.
            editUrl: "https://github.com/evinism/mistql/edit/docs/",
          },
          theme: {
            customCss: require.resolve("./src/css/custom.css"),
          },
          googleAnalytics: {
            trackingID: 'G-KXKSD351CT',
            anonymizeIP: true,
          },
        }),
      ],
    ],
    themeConfig:
      /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
      ({
        navbar: {
          title: "MistQL",
          logo: {
            alt: "MistQL logo",
            src: "img/logo.png",
          },
          items: [
            {
              type: "doc",
              docId: "intro",
              position: "left",
              label: "Tutorial",
            },
            {
              type: "doc",
              docId: "reference/overview",
              position: "left",
              label: "Reference",
            },
            {
              to: "/tryitout",
              position: "left",
              label: "Try it Out!",
            },
            {
              href: "https://github.com/evinism/mistql",
              label: "GitHub",
              position: "right",
            },
          ],
        },
        colorMode: {
          defaultMode: "light",
        },
        footer: {
          style: "dark",
          links: [
            {
              title: "Docs",
              items: [
                {
                  label: "Tutorial",
                  to: "/docs/intro",
                },
              ],
            },
            {
              title: "Community",
              items: [
                {
                  label: "Stack Overflow",
                  href: "https://stackoverflow.com/questions/tagged/mistql",
                },
                {
                  label: "Evin's Twitter",
                  href: "https://twitter.com/evinism",
                },
              ],
            },
            {
              title: "More",
              items: [
                {
                  label: "GitHub",
                  href: "https://github.com/evinism/mistql",
                },
              ],
            },
          ],
          copyright: `Copyright © ${new Date().getFullYear()} Evin Sellin and Vidora. Built with Docusaurus.`,
        },
        prism: {
          theme: lightCodeTheme,
          darkTheme: darkCodeTheme,
        },
      }),
  }
);
