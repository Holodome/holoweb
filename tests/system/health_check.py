import requests

def test_health_check():
    response = requests.get("localhost:8080/health_check")
    assert response.status_code == 200
