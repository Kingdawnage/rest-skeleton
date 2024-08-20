"use client";

import { useState } from "react";

const Page: React.FC = () => {
  const [data, setData] = useState<any>([]);
  const [error, setError] = useState<string | null>(null);

  function fetchApi() {
    fetch("http://127.0.0.1:8080/db")
      .then((response) => {
        console.log("Response: ", response);

        if (!response.ok) {
          throw new Error("Network response was not ok " + response.status);
        }
        return response.json();
      })
      .then((data) => {
        setData(data);
        setError(null);
      })
      .catch((error) => {
        setError(
          "There has been a problem with the fetch operation: " + error.message
        );
        setData(null);
      });
  }

  return (
    <div>
      <p>Fetch Data From Backend</p>
      <button onClick={fetchApi}>Fetch Data</button>
      {error && <p style={{ color: "red" }}>{error}</p>}
      {data && <pre>{JSON.stringify(data, null, 2)}</pre>}
    </div>
  );
};

export default Page;
