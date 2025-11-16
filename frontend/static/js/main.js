document.getElementById("searchForm").addEventListener("submit", async (e) => {
  e.preventDefault();

  const requestData = {
    number: document.getElementById("number").value || null,
    email: document.getElementById("email").value || null,
    full_name: document.getElementById("full_name").value || null,
    ip: document.getElementById("ip").value || null,
  };

  try {
    const response = await fetch("/search", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(requestData),
    });

    const result = await response.json();
    document.getElementById("result").innerHTML =
      `<pre>${JSON.stringify(result, null, 2)}</pre>`;
  } catch (error) {
    document.getElementById("result").innerHTML =
      `<p style="color: var(--rust)">Error: ${error.message}</p>`;
  }
});
