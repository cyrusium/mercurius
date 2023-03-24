def post(self, req):
  body = req.get_json()
  self.res(body["id"])