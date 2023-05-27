import express from "express";
import mysql from "mysql2";

//create app
const app = express();

//connect to db
const db = mysql.createConnection({
    host: "localhost",
    user: "root",
    password: "Maykol___123!!",
    database: "test"
});

//middleware to allow clinet send JSOn

app.use(express.json());


//connect to backend in browser

app.get('/', (req, res) => {
    res.json("hello this is the backend")
})

//connect to db and show data

app.get('/books', (req, res) => {

    //create query
    const q = "SELECT * FROM books"
    db.query(q, (err, data) => {

        //display error, or data 
        if (err) return res.json(err)
        return res.json(data)
    })
})

//create, post using node, express

app.post("/books", (req, res) => {

    //create query
    const q = "INSERT INTO books (`title`, `desc`, `cover`) VALUES (?)"

    const values = [
        req.body.title,
        req.body.desc,
        req.body.cover,
    ];

    db.query(q, [values], (err, data) => {
        //display error, or data 
        if (err) return res.json(err)
        return res.json("book's been created successfully");
    });
});

//run server in port 8800
app.listen(8800, () => {
    console.log("Connected to server")
})