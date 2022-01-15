import assert from "assert";
import { query } from ".";
import meta from "../../meta.json";
import packagejson from "../package.json";

// A random grab-bag of integration tests

describe("version", () => {
  it("is correct", () => {
    assert.deepStrictEqual(packagejson.version, meta.version);
  });
})

describe("index", () => {
  describe("#query", () => {
    it("functions on a very basic level", () => {
      assert.deepStrictEqual(query("@ | map @ + 1", [1, 2, 3]), [2, 3, 4]);
    });

    it("supports being passed falsy values for keys", () => {
      assert.deepStrictEqual(query("key + 1", { key: 0 }), 1);
    });

    it("allows a bunch unary operators in a row", () => {
      assert.deepStrictEqual(query("!!!!arg", { arg: true }), true);
      assert.deepStrictEqual(query("!!!!!true", { arg: true }), false);
      assert.deepStrictEqual(query("-num", { num: 0.5 }), -0.5);
      assert.deepStrictEqual(query("-5", {}), -5);
      assert.deepStrictEqual(query("--5", {}), 5);
    });

    it("doesn't allow object access of inherited properties", () => {
      assert.throws(() => query("@.length", [1, 2, 3]));
      assert.throws(() => query("@.map", [1, 2, 3]));
      assert.throws(() => query('(regex "hi").lastIndex', [1, 2, 3]));
    });

    it("allows complex expressions as part of object and array literals", () => {
      assert.deepStrictEqual(
        query('([-1, { isSpot: dog == "spot"}])', { dog: "spot" }),
        [-1, { isSpot: true }]
      );
    });
  });

  describe("indexing", () => {
    it("correctly grabs the first element", () => {
      assert.strictEqual(query("[1, 2, 3, 4, 5][0]", {}), 1);
    });

    it("returns null for empty arrays", () => {
      assert.strictEqual(query("[][0]", {}), null);
      assert.strictEqual(query("[][-1]", {}), null);
    });

    it("correctly grabs the last element", () => {
      assert.strictEqual(query("[1, 2, 3, 4, 5][-1]", {}), 5);
    });

    it("slices with first param missing", () => {
      assert.deepStrictEqual(query("[1, 2, 3, 4, 5][:-1]", {}), [1, 2, 3, 4]);
      assert.deepStrictEqual(query("[1, 2, 3, 4, 5][:3]", {}), [1, 2, 3]);
    });

    it("slices with second param missing", () => {
      assert.deepStrictEqual(query("[1, 2, 3, 4, 5][-2:]", {}), [4, 5]);
      assert.deepStrictEqual(query("[1, 2, 3, 4, 5][2:]", {}), [3, 4, 5]);
    });

    it("slices with first and last params", () => {
      assert.deepStrictEqual(query("[1, 2, 3, 4, 5][1:-1]", {}), [2, 3, 4]);
      assert.deepStrictEqual(query("[1, 2, 3, 4, 5][2:4]", {}), [3, 4]);
    });
  });

  describe("string parsing", () => {
    it("is a superset of JSON", () => {
      const bigOne = JSON.stringify([
        {
          _id: "6179c44ecd76228e2c5b01f0",
          index: 0,
          guid: "a7c128b6-793f-49ab-a691-ddd9c49617b7",
          isActive: false,
          balance: "$1,116.94",
          picture: "http://placehold.it/32x32",
          age: 37,
          eyeColor: "green",
          name: "Thelma Ferrell",
          gender: "female",
          company: "BULLZONE",
          email: "thelmaferrell@bullzone.com",
          phone: "+1 (872) 560-2522",
          address: "291 Lewis Place, Belvoir, Mississippi, 7297",
          about:
            "Et est voluptate nostrud ullamco mollit. Proident sunt cillum consequat esse occaecat anim voluptate Lorem anim mollit. Non tempor irure minim ut consectetur consectetur ex ipsum nostrud esse dolore. Eiusmod cupidatat proident labore tempor. Commodo cupidatat officia dolor fugiat.\r\n",
          registered: "2019-08-28T11:36:32 +07:00",
          latitude: 34.618981,
          longitude: -178.769145,
          tags: ["minim", "et", "do", "sit", "do", "magna", "ipsum"],
          friends: [
            {
              id: 0,
              name: "Dixon Heath",
            },
            {
              id: 1,
              name: "Kelsey Pugh",
            },
            {
              id: 2,
              name: "Shanna Oneil",
            },
          ],
          greeting: "Hello, Thelma Ferrell! You have 7 unread messages.",
          favoriteFruit: "banana",
        },
        {
          _id: "6179c44e5138f4d8cc8b8f06",
          index: 1,
          guid: "ccbe6878-aac3-44b2-908a-bba774c244fd",
          isActive: false,
          balance: "$2,744.60",
          picture: "http://placehold.it/32x32",
          age: 21,
          eyeColor: "green",
          name: "Vega Glover",
          gender: "male",
          company: "ZOARERE",
          email: "vegaglover@zoarere.com",
          phone: "+1 (921) 480-3985",
          address: "714 Harbor Court, Jacksonburg, North Dakota, 7321",
          about:
            "Nostrud cillum elit ut sit excepteur. Ullamco laboris Lorem nisi cillum dolor anim ullamco. Est officia fugiat proident elit anim in ipsum exercitation eu minim ipsum aute sint sunt. Ipsum consectetur sint aliquip nulla minim veniam voluptate reprehenderit. Aliqua et nulla in fugiat ad ullamco eu dolore nulla id ipsum.\r\n",
          registered: "2018-10-18T12:55:55 +07:00",
          latitude: -43.233126,
          longitude: -29.113182,
          tags: ["dolor", "ut", "ad", "enim", "do", "nisi", "elit"],
          friends: [
            {
              id: 0,
              name: "Elaine Gay",
            },
            {
              id: 1,
              name: "Miranda Nunez",
            },
            {
              id: 2,
              name: "Tamika Mcknight",
            },
          ],
          greeting: "Hello, Vega Glover! You have 1 unread messages.",
          favoriteFruit: "banana",
        },
        {
          _id: "6179c44e6f7f1ed403a46b50",
          index: 2,
          guid: "9ad365d7-7da9-460c-b293-ed677b932301",
          isActive: false,
          balance: "$3,575.24",
          picture: "http://placehold.it/32x32",
          age: 28,
          eyeColor: "green",
          name: "Dickerson Stein",
          gender: "male",
          company: "ZILLIDIUM",
          email: "dickersonstein@zillidium.com",
          phone: "+1 (873) 500-3594",
          address: "411 Seabring Street, Ruckersville, Alabama, 6571",
          about:
            "Veniam amet Lorem sunt non minim excepteur duis veniam officia reprehenderit ex consectetur ea. Consectetur est mollit laborum et culpa excepteur nisi. Ullamco nulla veniam voluptate sit proident sit eu veniam. Deserunt officia voluptate esse esse sit nulla sint aute laborum. Aute pariatur cillum nostrud tempor. Nulla dolore cupidatat aute id minim tempor sunt laboris deserunt aliqua culpa duis sunt.\r\n",
          registered: "2018-10-27T01:23:52 +07:00",
          latitude: -7.273399,
          longitude: -41.74266,
          tags: ["elit", "Lorem", "velit", "eiusmod", "id", "anim", "tempor"],
          friends: [
            {
              id: 0,
              name: "Knowles Tanner",
            },
            {
              id: 1,
              name: "Angel Ferguson",
            },
            {
              id: 2,
              name: "Leola Hickman",
            },
          ],
          greeting: "Hello, Dickerson Stein! You have 4 unread messages.",
          favoriteFruit: "banana",
        },
      ]);

      const strings: string[] = [
        '{"hello": "there"}',
        `{"hello": "\\nthere"}`,
        `{"hello": "\\\\nthere"}`,
        `{"hello": "\\\\sthere"}`,
        `{"hello": "\\bthere"}`,
        `{"hello": "\\\\bthere"}`,
        `{"hello": "\\\\bth\\"ere"}`,
        bigOne,
      ];
      strings.forEach((str) => {
        assert.deepStrictEqual(query(str, null), JSON.parse(str));
      });
    });
  });
});
