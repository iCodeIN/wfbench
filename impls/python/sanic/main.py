from sanic import Sanic
from sanic.response import text

app = Sanic(name=__name__)

@app.route('/demo')
async def demo(request):
    return text('Hello')

@app.route('/demo/<id:int>/<name>/other.html')
async def other(request, id, name):
    return text(f'Hello {name}! id:{id}')

@app.route('/demo/<id:int>/<name>/other.html', methods=['POST'])
async def other_post(request, id, name):
    return text(f'Hello {name}! id:{id}')

@app.route('/demo/<id:int>/<name>/test.html')
async def test(request, id, name):
    return text(f'Hello {name}! id:{id}')

@app.route('/demo/<id:int>/<name>/index.html')
async def index(request, id, name):
    return text(f'Hello {name}! id:{id}')

if __name__ == '__main__':
    app.run(host='127.0.0.1', port=8080, access_log=False, workers=48)
