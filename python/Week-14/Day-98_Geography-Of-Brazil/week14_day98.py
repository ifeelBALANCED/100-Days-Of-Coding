import httpx

BASE_URL = "https://servicodados.ibge.gov.br/api/v1/localidades/estados/"


def get_municipalities_count(state_acronym: str) -> int:
    response = httpx.get(url=f"{BASE_URL}/{state_acronym}/municipios")
    return len(response.json())
