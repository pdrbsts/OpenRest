# Chaves do OpenRest

## Comunicação com o web-service `SeriesWS` da AT

Para chamar o `SeriesWS` (ambiente de teste ou produção) precisamos de **dois
artefactos** para além da password do utilizador AT:

### `at_test_public.pem`

Chave **pública RSA** da Autoridade Tributária. Usada pelo crate `at-series`
para cifrar o `Nonce` no cabeçalho WS-Security UsernameToken (cifra híbrida
AT: Nonce → RSA/PKCS1, Password+Created → AES/ECB/PKCS5Padding com nonce).

Versão actual: `ChaveCifraPublicaAT2023.pem` (RSA-2048), distribuída
publicamente em `info.portaldasfinancas.gov.pt`.

### `at_test_client.pfx`

**Certificado de cliente** (PKCS#12) exigido pelo TLS mútuo dos endpoints
`:722` (teste) e `:422` (produção). Sem este certificado, o servidor da AT
aceita o handshake mas não responde (handshake_failure em TLS 1.2 / bad
record mac em TLS 1.3).

O servidor da AT só aceita certificados emitidos por uma das seguintes CAs:

* `C=PT, L=Lisboa, O=Autoridade Tributaria e Aduaneira, CN=AT Issuing CA1`
* `DC=local, DC=ritta, CN=DGITA Issuing CA1`

A AT publica um certificado de cliente de **testes** público (`TesteWebservices.pfx`,
password `TESTEwebservice`). **Atenção**: o cert distribuído publicamente em
2022 expirou em Março/2023 — a AT pode tê-lo rejeitar mesmo no ambiente de
teste. Em caso de erro `handshake_failure` ou `decryption failed`:

1. Confirma que o ficheiro existe e é legível.
2. Pede um certificado de testes actualizado em
   `info.portaldasfinancas.gov.pt` → Apoio ao Contribuinte → Faturação →
   Comunicação de Séries → Especificações Técnicas.
3. Em produção, gera-se um certificado próprio através do Portal das
   Finanças (Apoio ao Contribuinte → Webservices) e substitui-se este
   ficheiro pelo PFX emitido para o NIF real.

### Substituição em produção

No `openrest.toml`:

```toml
[at_series]
endpoint = "https://servicos.portaldasfinancas.gov.pt:422/SeriesWSService"
username = "<NIF>/<subutilizador>"
password = "<senha>"
public_key_path = "./keys/at_public_2023.pem"     # ou versão actual da AT
client_pfx_path = "./keys/<empresa>.pfx"
client_pfx_password = "<senha do PFX>"
```

## `at_test_client_cert.pem` / `at_test_client_key.pem`

Cert + chave do PFX em formato PEM (extraídos com `openssl pkcs12 -legacy`).
Não usados directamente pelo OpenRest — apenas úteis para debugging com
`openssl s_client -cert ... -key ...` quando precisamos de validar handshake
manualmente.

## `openrest_signing.pem` (na raiz do projecto)

Chave **privada** RSA-2048 do OpenRest para assinatura fiscal (Portaria
363/2010). É gerada automaticamente na primeira execução; em produção, a
chave pública correspondente é comunicada à AT durante o processo de
certificação.
