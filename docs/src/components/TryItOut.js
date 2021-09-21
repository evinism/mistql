import React, {useState} from 'react';
import styles from './TryItOut.module.css';
import mistql from 'mistql';

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

const initialQuery = 'events | filter type == "send_message" | groupby email | keys'
const initialData = JSON.stringify({
  events: [
    {
      type: 'like',
      email: 'harold@example.com',
      post_number: 5831
    },
    {
      type: 'send_message',
      email: 'flora@example.com',
      message: 'Hello, friend!',
      targetUser: 95813
    },
    {
      type: 'like',
      email: 'flora@example.com',
      post_number: 12385
    },
    {
      type: 'send_message',
      email: 'flora@example.com',
      message: 'I think you are cool!',
      targetUser: 95813
    },
    {
      type: 'send_message',
      email: 'william@example.com',
      message: 'You Too!',
      targetUser: 8381
    },
    {
      type: 'like',
      email: 'emma@example.com',
      post_number: 17245
    },
    {
      type: 'like',
      email: 'flora@example.com',
      post_number: 5831
    },
    {
      type: 'like',
      email: 'william@example.com',
      post_number: 5831
    },
    {
      type: 'like',
      email: 'pete@example.com',
      post_number: 17245
    },
  ]
}, null, 2);

export default function TryItOut() {
  const [dataText, setDataText] = useState(initialData);
  const [queryText, setQueryText] = useState(initialQuery);
  const {success, result, error} = runQuery(queryText, dataText);
  return (
    <section className={styles.tryitout}>
      <div className={styles.tryitoutinner}>
      <h2>Try it out!</h2>
      <div className={styles.querywrapper}>
        <label for="tryitoutquery">MistQL Query</label>
        <input id="tryitoutquery" className={styles.queryinput} type="text" value={queryText} onChange={(e) => setQueryText(e.target.value)} />
      </div>
      <div className={styles.datawrapper}>
        <label for="tryitoutdata">JSON Data</label>
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
