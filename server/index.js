const express = require("express");
import fs from "fs/promises";

const app = express();
const port = 25731;

const KEY = "retrosave";

app.get("/", async (req, res) => {
  const { username, filename, game } = req.headers;
  if (!username || !filename || !game) {
    res.status(400).send("Missing username, filename, or game headers.");
    return;
  }

  console.log(`Request from ${username} (${req.ip}) to download ${filename}`);
  const filePath = `${__dirname}/uploads/${username}/${game}/${filename}`;

  try {
    const fileContent = await fs.readFile(filePath);
    res.send(fileContent);
    console.log(`Sent file ${filename} to ${username} (${req.ip})`);
  } catch (error) {
    console.error(error);
    res.status(500).send("Internal Server Error");
  }
});

app.post("/", (req, res) => {
  let data = [];
  req.on("data", (chunk) => {
    data.push(chunk);
  });

  const { username, filename, game } = req.headers;

  if (!username || !filename) {
    res.status(400).send("Missing username or filename headers.");
    console.log(
      `Denied request from IP ${req.ip} due to missing headers: ${req.headers}`
    );
    return;
  }

  console.log(`Request from ${username} (${req.ip}) to upload ${filename}`);

  fs.mkdir(`uploads/${username}/${game}`, { recursive: true });

  req.on("end", () => {
    let buffer = Buffer.concat(data);

    fs.writeFile(
      `uploads/${username}/${game}/${filename}`,
      buffer,
      "base64",
      (err) => {
        if (err) {
          res.status(500).send("Error uploading file.");
          return;
        }
      }
    );

    res.send("File uploaded successfully.");
    console.log(
      `Finished file upload from ${username} (${req.ip}) to upload ${filename}`
    );
  });
});

app.listen(port, () => {
  console.log(`Retrosave server is running on port ${port}`);
});
