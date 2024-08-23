"use client";

import React, { useState } from "react";

interface FormData {
  name: string;
  password: string;
  age: number; // Use number instead of string for age
}

const Page: React.FC = () => {
  const [data, setData] = useState<any>([]);
  const [error, setError] = useState<string | null>(null);
  const [formData, setFormData] = useState<FormData>({
    name: "",
    password: "",
    age: 0,
  });

  function fetchApi() {
    fetch("http://127.0.0.1:8080/api/users")
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

  function handleInputChange(event: React.ChangeEvent<HTMLInputElement>) {
    const { name, value } = event.target;
    setFormData((prevData) => ({
      ...prevData,
      [name]: name === "age" ? parseInt(value, 10) : value,
    }));
  }

  function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();

    // Ensure that age is a number
    const dataToSend: FormData = {
      ...formData,
      age: Number(formData.age), // Ensure age is a number
    };

    fetch("http://127.0.0.1:8080/api/users", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(dataToSend),
    })
      .then((data) => {
        console.log("Data submitted: ", data);
        setFormData({ name: "", password: "", age: 0 });
        fetchApi();
      })
      .catch((error) => {
        console.error("Error:", error);
        setError(
          "There has been a problem with the fetch operation: " + error.message
        );
      });
  }

  return (
    <div>
      <p>Fetch Data From Backend</p>
      <button onClick={fetchApi}>Fetch Data</button>
      {error && <p style={{ color: "red" }}>{error}</p>}
      {data && <pre>{JSON.stringify(data, null, 2)}</pre>}

      <p>Submit Data</p>
      <form onSubmit={handleSubmit}>
        <div>
          <label>
            Name:
            <input
              type="text"
              name="name"
              value={formData.name}
              onChange={handleInputChange}
            />
          </label>
        </div>
        <div>
          <label>
            Password:
            <input
              type="password"
              name="password"
              value={formData.password}
              onChange={handleInputChange}
            />
          </label>
        </div>
        <div>
          <label>
            Age:
            <input
              type="number"
              name="age"
              value={formData.age}
              onChange={handleInputChange}
            />
          </label>
        </div>
        <button type="submit">Submit</button>
      </form>
    </div>
  );
};

export default Page;
