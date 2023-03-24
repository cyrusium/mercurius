def get(self, req):
  self.res('GET: Hello World')

def post(self, req):
  #self.redirect('/login')
  print(req.session)
  body = req.get_json()
  print(body.nome)
  self.res(req.get_json())

def put(self, req):
  self.res('PUT: Hello World')

def patch(self, req):
  self.res('PATCH: Hello World')

def delete(self, req):
  self.res('DELETE: Hello World')

def next():
  print('this should print after')