import express from "express";
import "dotenv/config";

const app = express();

const port = 6969;

app.use(express.json());

app.use(express.urlencoded({ extended: false }));

app.post("/webhook-sample", (req, res) => {
  const envUsername = process.env.USERNAME;
  const envPassword = process.env.PASSWORD;
  const reqBasicAuth64 = (req.headers.authorization || "").split(" ")[1] || "";
  const [loginReq, passwordReq] = Buffer.from(reqBasicAuth64, "base64")
    .toString()
    .split(":");
  if (envUsername !== loginReq || passwordReq !== envPassword) {
    console.log(
      `fornecido username ${loginReq} em vez de ${envUsername} ou ${passwordReq} em vez de ${envPassword}`
    );
    return res.status(401).json({
      status: "error",
      code: 401,
      message: "Usuário ou senhas incorretos",
    });
  }
  return res.status(200).json({
    status: "success",
    code: 200,
    message: "joia",
  });
});

app.listen(port, () => console.log(`App listening on port ${port}`));
