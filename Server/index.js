const express = require('express');
const bodyParser = require('body-parser');
const fs = require('fs');
const { FILE } = require('dns');
const app = express();

const FILENAME = "tasklist.json";
const PASSWORD = "password";

// create tasklist.json when the file doesn't exist
function operate_file() {
  var exampleData = {
    "example": "This is an example task"
  };
  try {
    if (!fs.existsSync(FILENAME)) {
      fs.writeFileSync(FILENAME, JSON.stringify(exampleData));
    }
  } catch(e) {
    console.error(e);
  }
}

function is_password_right(password) {
  return password == PASSWORD;
}

// when you start this program, tasklist.json is created
operate_file();

app.use(bodyParser.urlencoded({
    extended: true
}));
app.use(bodyParser.json());
app.use(express.static('public'));

app.listen(8000, () => {
});

app.post('/', (req, res) => {
  res.send("Thank you for reaching me out.");
});

app.get('/', (req, res) => {
  res.send("Thank you for reaching me out.");
});

// read json file and append newJson to the json data
app.post('/api/send', (req, res) => {
  const taskKey = req.body.taskKey;
  const task = req.body.task;
  const password = req.body.password;
  if (!is_password_right(password)) {
    res.send("Password is incorrect.");
    return;
  }
  const data = fs.readFileSync(FILENAME);
  const dict = JSON.parse(data);
  dict[[taskKey]] = task;
  fs.writeFileSync(FILENAME, JSON.stringify(dict));

  res.send("You added a new task.");
})

// return all json data
app.post('/api/list', (req, res) => {
  const password = req.body.password;
  if (!is_password_right(password)) {
    res.send("Password is incorrect.");
    return;
  }
  const data = fs.readFileSync(FILENAME);
  const dict = JSON.parse(data);
  res.json(dict);
});

// delete task by taskKey
app.post('/api/finish', (req, res) => {
  const taskKey = req.body.taskKey;
  const password = req.body.password;
  if (!is_password_right(password)) {
    res.send("Password is incorrect.");
    return;
  }
  const data = fs.readFileSync(FILENAME);
  const dict = JSON.parse(data);
  if (!dict.hasOwnProperty([taskKey])) {
    res.send("This task key doesn't exist.");
    return;
  }
  delete dict[taskKey];
  fs.writeFileSync(FILENAME, JSON.stringify(dict)); 
  res.send("You deleted the task.");
});

// delete json file
app.post('/api/reset', (req, res) => {
  const password = req.body.password;
  if (!is_password_right(password)) {
    res.send("Password is incorrect.");
    return;
  }
  try {
    fs.unlinkSync(FILENAME);
  } catch(e) {
    console.error(e);
    res.send("Something went wrong.")
  }
  operate_file();
  res.send("You reset all tasks.");
});