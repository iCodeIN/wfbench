const express = require('express')
const app = express()
const port = 8080

function helloHandler(req, res) {
    let id = parseInt(req.params.id, 10)
    let name = req.params.name
    res.send(`Hello ${name}! id:${id}`)
}

app.get('/demo', (req, res) => res.send('Hello'))

app.get('/demo/:id/:name/other.html', helloHandler)
app.post('/demo/:id/:name/other.html', helloHandler)
app.get('/demo/:id/:name/test.html', helloHandler)
app.get('/demo/:id/:name/index.html', helloHandler)

app.listen(port, () => console.log(`Listening on localhost:${port}`))
