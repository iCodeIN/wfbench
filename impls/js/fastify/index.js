const fastify = require('fastify')({
    logger: false
})

fastify.get('/demo', async (request, reply) => {
    return 'Hello'
})

async function helloHandler(request, reply) {
    let id = parseInt(request.params.id, 10)
    let name = request.params.name
    return `Hello ${name}! id:${id}`
}

fastify.get('/demo/:id/:name/other.html', helloHandler)
fastify.post('/demo/:id/:name/other.html', helloHandler)
fastify.get('/demo/:id/:name/test.html', helloHandler)
fastify.get('/demo/:id/:name/index.html', helloHandler)

fastify.listen(8080, (err, address) => {
    if (err) throw err
    console.log(`server listening on ${address}`)
})
