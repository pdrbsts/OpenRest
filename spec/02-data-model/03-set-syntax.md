# OpenRest — Sintaxe de Conjuntos (legado)

O WinREST usa uma sintaxe compacta para representar conjuntos de números (mesas atribuídas a um empregado, mesas de um local, postos de uma origem, etc.). OpenRest mantém esta sintaxe por compatibilidade conceptual e adiciona forma estruturada.

## Forma compacta

```
elementos ::= bloco ("," bloco)*
bloco     ::= num ( ":" num ( ":" num ( ":" num )? )? )?
              inicial : final : espaçamento : tamanho
```

| Forma | Significado | Exemplo |
|---|---|---|
| `n` | um elemento | `7` → `{7}` |
| `a:b` | intervalo contínuo | `1:5` → `{1,2,3,4,5}` |
| `a:b:e` | intervalo com passo `e` | `1:10:2` → `{1,3,5,7,9}` |
| `a:b:e:t` | sub-blocos de tamanho `t` espaçados por `e` | `1:50:10:2` → `{1,2,11,12,21,22,31,32,41,42}` |
| `bloco,bloco,…` | união | `1:5,20` → `{1..5, 20}` |

## Forma estruturada (OpenRest)

Aceita-se também o JSON:

```json
{
  "blocos": [
    {"de": 1, "ate": 5},
    {"de": 20},
    {"de": 1, "ate": 50, "passo": 10, "tamanho": 2}
  ]
}
```

Apresentação UI: editor com botões `Adicionar bloco`, `Remover`, com toggles para passo/tamanho. A forma compacta é gerada/reconstruída a partir da estruturada.

## Operações

- **Pertença**: `n ∈ S` — usado no controlo de acessos (este empregado pode aceder a esta mesa?).
- **Intersecção**: `S1 ∩ S2` — usado em exclusões (mesas do local intersectadas com mesas excluídas).
- **União**, **diferença** — para listagens e relatórios.

## Validação

- `a ≤ b`, `e ≥ 1`, `1 ≤ t ≤ e`.
- O sistema rejeita conjuntos com sobreposição interna inconsistente (avisos, mas aceita).
- Limite prático: 65.000 elementos por conjunto. Acima, recomenda-se splitting.
