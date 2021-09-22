import React, {useState} from 'react';
import styles from './TryItOut.module.css';
import mistql from 'mistql';
import examples from '../examples';

const defaultExample = examples['usersWithAMessageEvent'];

const runQuery = (query, data) => {
  try {
    return {
      success: true,
      result: mistql.query(query, JSON.parse(data))
    }
  } catch (error) {
    return {
      success: false,
      error,
    }
  }
}

export default function TryItOut() {
  const [dataText, setDataText] = useState(JSON.stringify(defaultExample.data, null, 2));
  const [queryText, setQueryText] = useState(defaultExample.query);
  const {success, result, error} = runQuery(queryText, dataText);
  return (
    <section className={styles.tryitout}>
      <div className={styles.tryitoutinner}>
      <h2>Try it out!</h2>

      <div className={styles.querywrapper}>
        <div className={styles.labelblock}>
          <label for="tryitoutquery">MistQL Query</label>
          <label>
            Example
            <select onChange={(e) => {
              const {data, query} = examples[e.target.value]
              setDataText(JSON.stringify(data, null, 2));
              setQueryText(query)
            }}>
              {Object.entries(examples).map(([value, example]) => <option key={value} value={value}>{example.name}</option>)}
            </select>
          </label>
        </div>
        <input id="tryitoutquery" className={styles.queryinput} type="text" value={queryText} onChange={(e) => setQueryText(e.target.value)} />
      </div>
      <div className={styles.datawrapper}>
        <div styles={styles.labelblock}>
          <label for="tryitoutdata">JSON Data</label>
        </div>
        <textarea id="tryitoutdata" className={styles.datafield} value={dataText} onChange={(e) => setDataText(e.target.value)}>
        </textarea>
      </div>
      <div className={styles.resultwrapper}>
        Result:
        <pre>
          {success ? JSON.stringify(result, null, 2) : error.message}
        </pre>
      </div>
      </div>
    </section>
  );
}
