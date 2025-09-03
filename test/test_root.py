def test_root(testclient):
    response = testclient.get("/")
    assert response.status_code == 200
    assert response.json() == {"message": "Hello World"}
