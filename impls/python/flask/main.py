from flask import Flask

app = Flask(__name__)

@app.route('/demo')
def demo():
    return 'Hello'

@app.route('/demo/<int:id>/<name>/other.html')
def other(id, name):
    return f'Hello {name}! id:{id}'

@app.route('/demo/<int:id>/<name>/other.html', methods=['POST'])
def other_post(id, name):
    return f'Hello {name}! id:{id}'

@app.route('/demo/<int:id>/<name>/test.html')
def test(id, name):
    return f'Hello {name}! id:{id}'

@app.route('/demo/<int:id>/<name>/index.html')
def index(id, name):
    return f'Hello {name}! id:{id}'
