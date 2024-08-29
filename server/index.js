const express = require("express");
import fs from "fs/promises";

const app = express();
const port = 25731;

app.get("/", (req, res) => {
  res.send(
    "Retrosave server is running; use POST on this endpoint to upload saves."
  );
});

app.post("/", (req, res) => {
  let data = [];
  req.on("data", (chunk) => {
    data.push(chunk);
  });

  const username = req.headers.name;
  const filename = req.headers.filename;

  if (!username || !filename) {
    res.status(400).send("Missing username or filename headers.");
    console.log(`Denied request from IP ${req.ip} due to missing headers.`);
    return;
  }

  console.log(`Request from ${username} (${req.ip}) to upload ${filename}`);

  fs.mkdir(`uploads/${username}`, { recursive: true });

  req.on("end", () => {
    let buffer = Buffer.concat(data);

    fs.writeFile(`uploads/${username}/${filename}`, buffer, "base64", (err) => {
      if (err) {
        res.status(500).send("Error uploading file.");
        return;
      }
    });

    res.send("File uploaded successfully.");
    console.log(
      `Finished file upload from ${username} (${req.ip}) to upload ${filename}`
    );
  });
});

app.listen(port, () => {
  console.log(`Retrosave server is running on port ${port}`);
});
