const { Pool, Client } = require("pg");

const credentials = {
  user: "sankar",
  host: "localhost",
  database: "sankar",
  password: "sankar",
  port: 5432,
};

// Connect with a connection pool.

async function poolDemo() {
  const pool = new Pool(credentials);
  const now = await pool.query("SELECT NOW()");
  await pool.end();

  return now;
}

// Connect with a client.

async function clientDemo() {
  const client = new Client(credentials);
  await client.connect();
  const now = await client.query("SELECT NOW()");
  await client.end();

  return now;
}

async function createUser(username) {
    const client = new Client(credentials);
  await client.connect();
  const newUser = await client.query(`INSERT INTO users(name) values('${username}')`);
  await client.end();

  return newUser;
}

async function insertExpense(title, paid, expenseType, userId) {
    const client = new Client(credentials);
  await client.connect();
  const newUser = await client.query(`INSERT INTO expenses(title, paid, expenseType, userId) values('${title}', '${paid}', '${expenseType}', '${userId}')`);
  await client.end();

  return newUser;
}
// Use a self-calling function so we can use async / await.

(async () => {
    const newUser = await insertExpense('paid to sankar', 40000, 'salary', 2);
    console.log('new', newUser);
})();